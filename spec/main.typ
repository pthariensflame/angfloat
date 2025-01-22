#import "@preview/ilm:1.4.0": *
#set text(lang: "en", font: "Bitter")
#show: ilm.with(
  paper-size: "us-letter",
  title: [#text(size: 33.5pt)[The Angfloat Specification]],
  author: "Laine Taffin Altman",
  date: datetime.today(),
  abstract: [#text(style: "italic", size: 14.1pt)[Version 1.0 Alpha 1]],
  preface: none,
  bibliography: bibliography("refs.yml"),
  figure-index: (enabled: true),
  table-index: (enabled: true),
  listing-index: (enabled: true),
  external-link-circle: false,
)
#show raw: set text(font: "Hack")

= Introduction

@ieee754
