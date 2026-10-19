#!/usr/bin/env ruby
# frozen_string_literal: true

# Met à jour la ligne « Suite : [titre](page.md) » à la fin de chaque chapitre, d'après
# l'ordre des pages de src/catalog.rs : chaque chapitre mène au suivant. Seul le lien
# est remplacé quand il ne mène pas au bon chapitre ; la suite de la phrase reste.
# Les pages traduites aussi : « Next: » dans content/en/, « Siguiente: » dans
# content/es/ (avec le titre traduit du chapitre suivant, s'il l'est).
#
#   ruby tools/suite.rb          # réécrit les lignes
#   ruby tools/suite.rb --check  # signale celles qui ne sont pas à jour

Encoding.default_external = Encoding::UTF_8
check = ARGV.include?("--check")
catalog = File.read(File.join(__dir__, "../src/catalog.rs"))
tracks = catalog.scan(/pages!\("(\w+)":\s*(.*?)\)/m).map do |track, list|
  [track, list.scan(/"([\w-]+)"/).flatten]
end
abort "aucun parcours trouvé dans src/catalog.rs" if tracks.empty?

content = File.join(__dir__, "../content")
wrong = 0
[["", "Suite :"], ["en/", "Next:"], ["es/", "Siguiente:"]].each do |lang, word|
tracks.each do |track, pages|
  pages.each_with_index do |page, i|
    nxt = pages[i + 1] or next
    file = File.join(content, lang, track, "#{page}.md")
    next unless File.exist?(file)

    text = File.read(file)
    target = [File.join(content, lang, track, "#{nxt}.md"), File.join(content, track, "#{nxt}.md")].find { |f| File.exist?(f) }
    title = File.read(target)[/\A# (.+)$/, 1]
    line = "#{word} [#{title}](#{nxt}.md)"
    lines = text.lines
    at = lines.rindex { |l| l.start_with?(word) }
    if at.nil?
      next if %w[aide-memoire glossaire].include?(page)

      wrong += 1
      puts "#{lang}#{track}/#{page} : pas de ligne « #{word} »" if check
      next if check

      lines << "\n" unless lines.last&.end_with?("\n")
      lines << "\n#{line}\n"
    else
      # Le paragraphe « Suite : … », qui peut tenir sur plusieurs lignes.
      stop = (at...lines.size).find { |k| lines[k].strip.empty? } || lines.size
      paragraph = lines[at...stop].join
      if paragraph[/\]\(([\w-]+)\.md[)#]/, 1] == nxt
        next
      end

      wrong += 1
      puts "#{lang}#{track}/#{page} : « #{paragraph.strip.tr("\n", " ")} » ne mène pas à #{nxt}.md" if check
      lines[at...stop] = ["#{line}\n"]
    end
    File.write(file, lines.join) unless check
  end
end
end
puts check ? "#{wrong} ligne(s) à mettre à jour" : "#{wrong} ligne(s) mise(s) à jour"
exit(check && wrong.positive? ? 1 : 0)
