#!/usr/bin/env ruby
# frozen_string_literal: true

# Runs every piece of code of the documentation and compares its output.
#
#   ruby tools/verifier.rb                      # every page
#   ruby tools/verifier.rb content/rust/*.md    # some pages
#
# A block ```ruby, ```rust or ```rythme is run (Rust blocks without `fn main` are
# wrapped in one). Add `norun` after the language to skip it (```rust norun).
# A ```sortie block right after it gives the exact expected output.
# Tools: `ruby`, `rustc` and `rythme` (or the paths in $RUBY, $RUSTC, $RYTHME).

Encoding.default_external = Encoding::UTF_8
Encoding.default_internal = Encoding::UTF_8

require "open3"
require "tmpdir"

RUBY = ENV.fetch("RUBY", "ruby")
RUSTC = ENV.fetch("RUSTC", "rustc")
RYTHME = ENV.fetch("RYTHME", "rythme")
RUNNABLE = %w[ruby rust rythme].freeze

Block = Struct.new(:file, :line, :lang, :flags, :code, :expected)

def blocks(file)
  found = []
  lines = File.readlines(file, chomp: true)
  i = 0
  while i < lines.size
    if (m = lines[i].match(/\A```(\w+)\s*(.*)\z/))
      start = i + 1
      lang = m[1]
      flags = m[2].split
      body = []
      i += 1
      while i < lines.size && lines[i] != "```"
        body << lines[i]
        i += 1
      end
      raise "#{file}:#{start}: block never closed" if i >= lines.size

      if lang == "sortie"
        prev = found.last
        if prev && prev.expected.nil? && prev.instance_variable_get(:@end) == start - 2
          prev.expected = body.join("\n")
        end
      else
        b = Block.new(file, start, lang, flags, body.join("\n"), nil)
        b.instance_variable_set(:@end, i + 1)
        found << b
      end
    end
    i += 1
  end
  found
end

def run(block, dir)
  case block.lang
  when "ruby"
    path = File.join(dir, "main.rb")
    File.write(path, block.code + "\n")
    Open3.capture3({ "LANG" => "C.UTF-8", "LC_ALL" => "C.UTF-8" }, RUBY, path)
  when "rythme"
    path = File.join(dir, "main.ry")
    File.write(path, block.code + "\n")
    Open3.capture3({ "RYTHME_LANG" => "fr" }, RYTHME, "run", path)
  when "rust"
    code = block.code
    code = "fn main() {\n#{code}\n}" unless code.match?(/^\s*(pub\s+)?(async\s+)?fn main\s*\(/)
    path = File.join(dir, "main.rs")
    File.write(path, "#![allow(unused, dead_code)]\n" + code + "\n")
    out, err, status = Open3.capture3(RUSTC, "--edition", "2024", "-o", File.join(dir, "main"), path)
    return [out, err, status] unless status.success?

    Open3.capture3(File.join(dir, "main"))
  end
end

files = ARGV.empty? ? Dir["content/**/*.md"].sort : ARGV
failures = 0
count = 0
files.each do |file|
  blocks(file).each do |block|
    next unless RUNNABLE.include?(block.lang)
    next if block.flags.include?("norun")

    count += 1
    out, err, status = Dir.mktmpdir { |dir| run(block, dir) }
    where = "#{block.file}:#{block.line} (#{block.lang})"
    if !status.success?
      failures += 1
      puts "✗ #{where}: failed\n#{err.lines.first(25).join}"
    elsif block.expected && out.rstrip != block.expected.rstrip
      failures += 1
      puts "✗ #{where}: wrong output\n--- expected\n#{block.expected}\n--- got\n#{out}"
    end
  end
end
puts "#{count} blocks run, #{failures} failure(s)"
exit(failures.zero? ? 0 : 1)
