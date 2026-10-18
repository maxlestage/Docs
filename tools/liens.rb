#!/usr/bin/env ruby
# frozen_string_literal: true

# Vérifie l'export statique : chaque lien interne (href, src) de chaque page HTML mène
# à un fichier qui existe, et chaque ancre (#…) à un id de la page visée.
#
#   ruby tools/liens.rb dist /Docs

require "set"

Encoding.default_external = Encoding::UTF_8
dir = ARGV.fetch(0, "dist")
base = ARGV.fetch(1, "").chomp("/")
pages = Dir[File.join(dir, "**", "*.html")]
abort "aucune page dans #{dir}" if pages.empty?

ids = Hash.new { |h, file| h[file] = File.read(file).scan(/\sid="([^"]+)"/).flatten.to_set }
failures = []
checked = 0
pages.each do |page|
  # Le code montré dans les pages (`<code>`, `<pre>`) peut contenir `href="…"` : ce
  # n'est pas un lien.
  html = File.read(page).gsub(%r{<pre\b.*?</pre>|<code\b.*?</code>}m, "")
  html.scan(/(?:href|src)="([^"]+)"/).flatten.uniq.each do |url|
    next if url.match?(%r{\A(https?:|mailto:|data:|//)})

    path, anchor = url.split("#", 2)
    path = path.split("?", 2).first
    target =
      if path.nil? || path.empty?
        page
      else
        unless path.start_with?("#{base}/")
          failures << "#{page}: #{url} n'est pas derrière #{base}/"
          next
        end
        file = File.join(dir, path.delete_prefix(base))
        file = File.join(file, "index.html") if path.end_with?("/")
        file
      end
    checked += 1
    if !File.file?(target)
      failures << "#{page}: #{url} → #{target} n'existe pas"
    elsif anchor && !anchor.empty? && target.end_with?(".html") && !ids[target].include?(anchor)
      failures << "#{page}: #{url} → pas d'ancre ##{anchor}"
    end
  end
end
puts failures
puts "#{pages.size} pages, #{checked} liens vérifiés, #{failures.size} problème(s)"
exit(failures.empty? ? 0 : 1)
