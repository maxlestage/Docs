#!/usr/bin/env ruby
# frozen_string_literal: true

# Met à jour la ligne « Suite : [titre](page.md) » à la fin de chaque chapitre, d'après
# l'ordre des pages de src/catalog.rs : chaque chapitre mène au suivant. Seul le lien
# est remplacé quand il ne mène pas au bon chapitre ; la suite de la phrase reste.
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

wrong = 0
tracks.each do |track, pages|
  pages.each_with_index do |page, i|
    nxt = pages[i + 1] or next
    file = File.join(__dir__, "../content", track, "#{page}.md")
    text = File.read(file)
    title = File.read(File.join(__dir__, "../content", track, "#{nxt}.md"))[/\A# (.+)$/, 1]
    line = "Suite : [#{title}](#{nxt}.md)"
    lines = text.lines
    at = lines.rindex { |l| l.start_with?("Suite :") }
    if at.nil?
      next if %w[aide-memoire glossaire].include?(page)

      wrong += 1
      puts "#{track}/#{page} : pas de ligne « Suite : »" if check
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
      puts "#{track}/#{page} : « #{paragraph.strip.tr("\n", " ")} » ne mène pas à #{nxt}.md" if check
      lines[at...stop] = ["#{line}\n"]
    end
    File.write(file, lines.join) unless check
  end
end
puts check ? "#{wrong} ligne(s) à mettre à jour" : "#{wrong} ligne(s) mise(s) à jour"
exit(check && wrong.positive? ? 1 : 0)
