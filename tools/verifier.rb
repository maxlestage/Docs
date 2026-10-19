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
# Each block runs in its own empty folder: it may write files.
# A block inside a quote (`> ```ruby`, a solution) is run too.
# Tools: `ruby`, `rustc`, `rythme`, `python3`, `go`, `node`, and TypeScript in tools/ts
# (`npm ci --prefix tools/ts`), or the paths in $RUBY, $RUSTC, $RYTHME, $PYTHON, $GO,
# $NODE, $TSC. Go blocks are whole programs (`package main`); JavaScript and TypeScript
# blocks are ES modules (top-level `await` works). TypeScript is type-checked with
# `--strict` before it runs: a type error is a failure.

Encoding.default_external = Encoding::UTF_8
Encoding.default_internal = Encoding::UTF_8

require "open3"
require "tmpdir"

RUBY = ENV.fetch("RUBY", "ruby")
RUSTC = ENV.fetch("RUSTC", "rustc")
RYTHME = ENV.fetch("RYTHME", "rythme")
PYTHON = ENV.fetch("PYTHON", "python3")
GO = ENV.fetch("GO", "go")
NODE = ENV.fetch("NODE", "node")
# TypeScript et les types de Node : `npm ci --prefix tools/ts`.
TS_DIR = File.expand_path("ts/node_modules", __dir__)
TSC = ENV.fetch("TSC", File.join(TS_DIR, ".bin/tsc"))
RUNNABLE = %w[ruby rust rythme python go javascript typescript].freeze

Block = Struct.new(:file, :line, :lang, :flags, :code, :expected)

def blocks(file)
  found = []
  # Le code peut être dans une citation (une solution d'exercice : `> ```ruby`) :
  # le `>` du début des lignes n'est alors pas du code.
  lines = File.readlines(file, chomp: true)
  i = 0
  while i < lines.size
    if (m = lines[i].match(/\A(>\s?)?```(\w+)\s*(.*)\z/))
      quote = m[1] ? />\s?/ : nil
      unquote = ->(l) { quote ? l.sub(/\A>\s?/, "") : l }
      start = i + 1
      lang = m[2]
      flags = m[3].split
      body = []
      i += 1
      while i < lines.size && unquote.(lines[i]) != "```"
        body << unquote.(lines[i])
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
    Open3.capture3({ "LANG" => "C.UTF-8", "LC_ALL" => "C.UTF-8" }, RUBY, path, chdir: dir)
  when "rythme"
    path = File.join(dir, "main.ry")
    File.write(path, block.code + "\n")
    # Les pages traduites (content/en/…, content/es/…) montrent les messages de Rythme
    # dans leur langue.
    lang = block.file[%r{content/(en|es)/}, 1] || "fr"
    Open3.capture3({ "RYTHME_LANG" => lang }, RYTHME, "run", path, chdir: dir)
  when "python"
    path = File.join(dir, "main.py")
    File.write(path, block.code + "\n")
    Open3.capture3({ "PYTHONIOENCODING" => "utf-8", "PYTHONHASHSEED" => "0" }, PYTHON, path, chdir: dir)
  when "go"
    path = File.join(dir, "main.go")
    File.write(path, block.code + "\n")
    Open3.capture3({ "GO111MODULE" => "off", "GOFLAGS" => "" }, GO, "run", path, chdir: dir)
  when "javascript"
    path = File.join(dir, "main.mjs")
    File.write(path, block.code + "\n")
    Open3.capture3(NODE, path, chdir: dir)
  when "typescript"
    path = File.join(dir, "main.mts")
    File.write(path, block.code + "\n")
    out, err, status = Open3.capture3(
      TSC, "--strict", "--target", "es2022", "--module", "nodenext", "--moduleResolution", "nodenext",
      "--types", "node", "--typeRoots", File.join(TS_DIR, "@types"), "--outDir", File.join(dir, "out"), path,
      chdir: dir
    )
    return [out + err, out + err, status] unless status.success?

    Open3.capture3(NODE, File.join(dir, "out", "main.mjs"), chdir: dir)
  when "rust"
    code = block.code
    code = "fn main() {\n#{code}\n}" unless code.match?(/^\s*(pub\s+)?(async\s+)?fn main\s*\(/)
    path = File.join(dir, "main.rs")
    File.write(path, "#![allow(unused, dead_code)]\n" + code + "\n")
    out, err, status = Open3.capture3(RUSTC, "--edition", "2024", "-o", File.join(dir, "main"), path)
    return [out, err, status] unless status.success?

    Open3.capture3(File.join(dir, "main"), chdir: dir)
  end
end

files = ARGV.empty? ? Dir["content/**/*.md"].sort : ARGV
jobs = files.flat_map { |file| blocks(file) }
            .select { |block| RUNNABLE.include?(block.lang) && !block.flags.include?("norun") }

# Les blocs tournent en parallèle (un par processeur, ou $JOBS), chacun dans son dossier.
require "etc"
queue = Queue.new
jobs.each_with_index { |block, k| queue << [block, k] }
results = Array.new(jobs.size)
workers = Integer(ENV.fetch("JOBS", Etc.nprocessors)).clamp(1, 32).times.map do
  Thread.new do
    loop do
      block, k = begin
        queue.pop(true)
      rescue ThreadError
        break
      end
      results[k] = Dir.mktmpdir { |dir| run(block, dir) }
    end
  end
end
workers.each(&:join)

failures = 0
jobs.each_with_index do |block, k|
  out, err, status = results[k]
  where = "#{block.file}:#{block.line} (#{block.lang})"
  if !status.success?
    failures += 1
    puts "✗ #{where}: failed\n#{err.lines.first(25).join}"
  elsif block.expected && out.rstrip != block.expected.rstrip
    failures += 1
    puts "✗ #{where}: wrong output\n--- expected\n#{block.expected}\n--- got\n#{out}"
  end
end
puts "#{jobs.size} blocks run, #{failures} failure(s)"
exit(failures.zero? ? 0 : 1)
