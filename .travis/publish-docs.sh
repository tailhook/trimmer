#!/bin/bash

publish_docs() {
    pip install sphinx docutils ghp-import --user &&
    ~/.local/bin/sphinx-build -b html doc doc/_build &&
    ~/.local/bin/ghp-import -n doc/_build/html &&
    git push -fq https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
} && publish_docs
