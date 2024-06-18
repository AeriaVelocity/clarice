# Clarice

> ![NOTE]
> Clarice is not implemented yet. *sad face*
> Still, though, for the sake of the README, I'll act like it is.

Clarice is a general-purpose natural-language functional/declarative programming
language built for simplicity, conciseness and clarity, using programming
statements that flow like English sentences rather than feeling like programming
in something traditional like C or Python.

Clarice aims to minimise symbols and promote use of natural English words,
similarly to Python, but with an even greater emphasis on prose-like code.

Clarice is incredibly weakly typed and does not support any form of type
annotations, but the Clarice interpreter ensures type safety and garbage
collection.

## Examples

### Hello, World

```clarice
with helloMsg as "Hello, world!" print helloMsg

# > Hello, world!
```

### Markdown to HTML

```clarice
using Markdown from Clarice/Extra

let description as """# Clarice

Clarice is a natural-language functional/declarative programming language built
for simplicity, conciseness and clarity, using programming statements that flow
like English sentences rather than feeling like programming in something
traditional like C or Python.
"""

with Markdown.ConvertHTML as htmlize do
    htmlize description to "ClariceDescription.html"
```

## Note about Usage/Contribution/Future/Other Stuff

Clarice is still very much ~~in alpha stages~~ NOT IMPLEMENTED. If you wish to
collaborate with me on Clarice's development, please feel free to send pull
requests.

When it's ready, the Clarice programming language will only include include the
Clarice interpreter. Eventually, I plan to write an LSP server for Clarice and a
Clarice-to-Python transpiler.

## Licence

Clarice is licensed under the GNU Lesser General Public License v3.0. This free
software licence requires improvements you make to Clarice to remain under the
LGPL. This includes improvements to the Clarice interpreter, the Clarice
Standard Library and the modules developed and maintained as part of the Clarice
project.

Integrating Clarice or the Clarice interpreter into your proprietary projects or
systems without making your own projects free/libre is allowed.

For more information, see
[the GNU Lesser General Public License](https://www.gnu.org/licenses/lgpl-3.0.html).

## Acknowledgements

- [Python](https://www.python.org/) for providing an idea of what I want to do
with Clarice.
- [Haskell](https://www.haskell.org/) for providing a good template for
functional and declarative programming.

## Contributing

Please see
["Note about Usage/Contribution/Future/Other Stuff"](#note-about-usagecontributionfutureother-stuff) above.
