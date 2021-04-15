default: tests/fixtures/english-wordnet-2020.xml

tests/fixtures/english-wordnet-2020.xml:
	mkdir -p tests/fixtures
	curl https://en-word.net/static/english-wordnet-2020.xml.gz | gunzip > $@
