# hburger

A command to turn long strings and paths into readable, recognizable, fixed-length strings, as well as a minimal shell setup to keep your prompt length in check.

## The Problem

In the context of user interfaces, dealing with variable length, unbounded strings can be troublesome.

As a prime example, let's consider the popular choice of populating a shell prompt to include the path to the current working directory:

* Since the path is unbounded in length, terminals often have to resort to line wrapping when the prompt gets too long, resulting in suboptimal user experience.
  This is especially a problem when working with small-sized terminals (common when using a terminal multiplexer) and/or deeply nested directory structures.
* The variability of the prompt length causes the initial cursor position to move around depending on the current working directory, which can be disrupting for the user.

Here is an example session of a typical prompt that displays user, host, and current path information (with line wrapping denoted by `|`):

```
                                                                                        |
alice@org-server-c[~/]% cd supercalifragilisticexpialidocious                           |
alice@org-server-c[~/supercalifragilisticexpialidocious]% cd foo                        |
alice@org-server-c[~/supercalifragilisticexpialidocious/foo]% cd honorificabilitudinitat|
ibus                                                                                    |
alice@org-server-c[~/supercalifragilisticexpialidocious/foo/honorificabilitudinitatibus]|
% cd ../../foobar                                                                       |
alice@org-server-c[~/supercalifragilisticexpialidocious/foobar]% cd honorificabilitudini|
tatibus                                                                                 |
alice@org-server-c[~/supercalifragilisticexpialidocious/foobar/honorificabilitudinitatib|
us]% cd foo                                                                             |
alice@org-server-c[~/supercalifragilisticexpialidocious/foobar/honorificabilitudinitatib|
us/foo]% cd bar                                                                         |
alice@org-server-c[~/supercalifragilisticexpialidocious/foobar/honorificabilitudinitatib|
us/foo/bar]% cd baz                                                                     |
alice@org-server-c[~/supercalifragilisticexpialidocious/foobar/honorificabilitudinitatib|
us/foo/bar/baz]%                                                                        |
```

As you can see, line wrapping and variable start cursor position can be disorienting for the user.

A possible solution to this problem involves transforming strings to make them shorter but still recognizable.
This is where hasburgers come into play.

## The Hashburger

A hashburger is a fixed-length, user-friendly representation of a long string.

For example, a hashburger of the string

```
supercalifragilisticexpialidocious
```

is

```
superc390ocious
```

A hashburger is composed of:

* *Left bun*: the first characters of the string
* *Right bun*: the last characters of the string
* *Hashpatty*: a hash of the _patty_, the string obtained by stripping the buns from the original string

Below we visually denote left bun (`<`), right bun (`>`), patty (`~`), and hashpatty (`=`) for the example string:
```
supercalifragilisticexpialidocious
<<<<<<~~~~~~~~~~~~~~~~~~~~~~>>>>>>

superc390ocious
<<<<<<===>>>>>>
```

Hashburgers are:

* **Fixed-length**: By construction, the length of a hashburger is always the sum of the lengths of the buns and hashpatty, no matter how long the original string is.
* **Distinguishable**: Even in the case where two different strings share the same left and right bun, the hashpatty serves as a best-effort last resort to tell them apart.
* **Adjustable**: You can tune the length of left bun, right bun, and hashpatty to fit your use case. You can also decide whether to pad short strings to maintain uniform lengths regardless of the lengths of the original strings.

Moreover, depending on the situation, hashburger can be:

* **Readable**: Preserving both content and position of the buns can make hashburgers generally easy to read. Having the hashpatty as a numeric string can also help in visually distinguishing it from the buns, which might mostly contain letters.
* **Recognizable**: The first and last part of a string is preserved, which can usually be enough to recognize the original string.

## Hashburgers for Paths

While compressing a string as a plain hashburger can be enough in some situations, other use cases might benefit from a more careful approach.

For example, a hashburger of the path string

```
/supercalifragilisticexpialidocious/foo/honorificabilitudinitatibus
```

is

```
/super649atibus
```

While the string is now fixed in length, information like number of components and their names has been lost, which is suboptimal.

To address this, we can compute the hashburger of each component separately and then put them back together, obtaining the much more informative string
```
/superc390ocious/foo/honori978atibus
```

We can also pad short strings to maintain equal spacing between `/`.
In this way, path strings with same number of components will retain the same length, providing a more uniform look that can enhance user experience:
```
/superc390ocious/foo            /honori978atibus
/superc390ocious/foobar         /honori978atibus
/superc390ocious/lopado664erygon/honori978atibus
```

Another thing to note is that the number of components in a path is unbounded.
Consider the path string

```
/supercalifragilisticexpialidocious-1/supercalifragilisticexpialidocious-2/supercalifragilisticexpialidocious-3/supercalifragilisticexpialidocious-4/supercalifragilisticexpialidocious-5/supercalifragilisticexpialidocious-6
```

Simply turning the individual components to hashburgers will still result in an exceedingly long string:

```
/superc016ious-1/superc016ious-2/superc016ious-3/superc016ious-4/superc016ious-5/superc016ious-6
```

We can achieve fixed length by specifying a limit on the number of components displayed and omitting the middle components, analogously to how we omit the central part of strings with hashpatties.
The underlying assumption is that the path components we usually care about are at the start and/or end of the path.
A possible resulting string would then be

```
/superc016ious-1/superc016ious-2:superc016ious-5/superc016ious-6
```

With a `:` separator informing us that some components have been omitted.
In this way, we manage to have path strings that are both bounded and informative.

## Hashburger Shell Prompt

We can leverage hashburgers to come up with a shell prompt that is nice to work with.
In particular:

* We turn user and host into hashburgers
* We turn each component in the current path into a hashburger of moderate length, ensuring readability and recognizability without giving up on conciseness
* We set a limit on the number of displayed path components

Here is the previous example with a reworked prompt that features hasburgers (with line wrapping denoted by `|`):

```
a7e@o1c[~/]% cd supercalifragilisticexpialidocious                                      |
a7e@o1c[~/supe17ious]% cd foo                                                           |
a7e@o1c[~/supe17ious/foo       ]% cd honorificabilitudinitatibus                        |
a7e@o1c[~/supe17ious/foo       /hono21ibus]% cd ../../foobar                            |
a7e@o1c[~/supe17ious/foobar    ]% cd honorificabilitudinitatibus                        |
a7e@o1c[~/supe17ious/foobar    /hono21ibus]% cd foo                                     |
a7e@o1c[~/supe17ious/foobar    /hono21ibus/foo       ]% cd bar                          |
a7e@o1c[~/supe17ious/foo       :foo       /bar       ]% cd baz                          |
a7e@o1c[~/supe17ious/foo       :bar       /baz       ]%                                 |
```

As you can see, we no longer incur in annoying line wrapping, making the user experience linear and pleasant.

Parameters such as bun length, hashpatty length, and number of components can be tuned to suit one's preference.
In the example above, we went for very small hashburgers for the user and host, since these values are likely to be recognizable anyways.
Hashburgers of path components are larger to make them more readable and recognizable in front of being derived from arbitrary directory names.
Finally, the number of components is a tradeoff between conciseness and completeness of the displayed path.


## Installation

### `hburger` command

Download the tarball and extract:

```
wget https://github.com/niqodea/hburger/releases/download/v0.2.0/hburger-v0.2.0-x86_64-unknown-linux-gnu.tar.gz
tar -xzf hburger-v0.2.0-x86_64-unknown-linux-gnu.tar.gz
```

then `cp` the `hburger` binary in the `bin` directory.

1. **Global Installation**:
   ```
   sudo cp hburger /usr/bin
   ```

2. **Local Installation**:
   First, ensure `~/.local/bin` is in your `PATH`. Then:
   ```
   cp hburger ~/.local/bin
   ```

After installation, run the following to make sure everything went smoothly and to get started with the command:
```sh
hburger --help
```

### Hashburger Prompt

We also include a `prompt.sh` script to turn your shell prompt into a simple hashburger prompt.
Run
```sh
source prompt.sh  # to use it in the current shell
cat prompt.sh >> ~/.bashrc  # to use it in all new bash sessions
cat prompt.sh >> ~/.zshrc  # to use it in all new zsh sessions
```

The `hburger` command must be installed for the hashburger prompt to work.
