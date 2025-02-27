'use strict';

module.exports = {
  source: {
    include: ["../pkg", "../typescript/dist"],
    includePattern: ".js$",
    excludePattern: "(node_modules/|docs)"
  },
  plugins: ["plugins/markdown"],
  markdown: {
    gfm: true
  },
  opts: {
    template: "./node_modules/clean-jsdoc-theme",
    encoding: "utf8",
    tutorials: "../tutorials",
    destination: ".",
    recurse: true,
    verbose: true,
    readme: "../README.md"
  },
  templates: {
    cleverLinks: true,
    monospaceLinks: true,
    systemName: "MathLikeAnim-rs Project",
    footer: "A project by MathItYT",
    outputSourceFiles: true
  }
}