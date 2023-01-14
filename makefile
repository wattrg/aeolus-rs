install:
	cargo install --path src/aeolus/ --root ${AEOLUS_HOME}
	cp -r resources ${AEOLUS_HOME}
