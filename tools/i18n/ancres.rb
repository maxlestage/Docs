#!/usr/bin/env ruby
# frozen_string_literal: true

# Les liens vers le glossaire des pages traduites : l'ancre française
# (`glossaire.md#chaine-de-caracteres`) devient celle du glossaire traduit (`#string`),
# d'après `glossaire-en.tsv` et `glossaire-es.tsv` (ancre française, ancre traduite).
#
#   ruby tools/i18n/ancres.rb          réécrit les pages
#   ruby tools/i18n/ancres.rb --check  échoue s'il reste une ancre à réécrire

Encoding.default_external = Encoding::UTF_8
root = File.expand_path("../..", __dir__)
check = ARGV.include?("--check")
left = []
%w[en es].each do |lang|
  map = File.readlines(File.join(__dir__, "glossaire-#{lang}.tsv"), chomp: true, encoding: "UTF-8")
            .map { |line| line.split("\t") }.select { |pair| pair.size == 2 }.to_h
  targets = map.values.to_h { |v| [v, true] }
  Dir[File.join(root, "content", lang, "**", "*.md")].sort.each do |file|
    text = File.read(file)
    fixed = text.gsub(/glossaire\.md#([a-z0-9-]+)/) do
      anchor = Regexp.last_match(1)
      new = targets[anchor] ? anchor : map.fetch(anchor, anchor)
      "glossaire.md##{new}"
    end
    next if fixed == text

    if check
      left << file.delete_prefix("#{root}/")
    else
      File.write(file, fixed)
      puts "réécrit : #{file.delete_prefix("#{root}/")}"
    end
  end
end
abort("ancres françaises vers le glossaire :\n#{left.join("\n")}") unless left.empty?
