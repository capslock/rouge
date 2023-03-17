# rouge_gui

Crate that implements an immediate-mode style text-mode gui useable with
bracket-lib or bracket-bevy. Very rough and very incomplete, but I've gotten
some good mileage out of it already. It can simplify some of the boilerplate of
creating windows and handling input.

## Features

* `bevy`: Makes the crate compatible with `bracket-bevy`. Otherwise uses
  `bracket-lib`.