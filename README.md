# mdbooks-pagebreaks

A mdbook preprocessor that replaces `{{---}}` with explicit print page breaks in HTML.

## Example

The sections `Title`, `Section 1`, `Section 2`, and `Section 3` will all be printed on seperate pages.

```markdown
# Title

Donec condimentum eros ante, eu mollis orci suscipit sit amet. Morbi semper mi turpis, eu scelerisque nibh scelerisque id.

{{---}}

## Section 1

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed leo elit, interdum nec ante non, malesuada fringilla ligula. Vestibulum bibendum lobortis sapien, nec sodales metus porttitor quis. Vivamus fringilla luctus tincidunt. Pellentesque viverra sagittis euismod.

{{---}}

## Section 2

Proin tortor risus, viverra et tortor viverra, pretium luctus lorem. Nulla facilisi. Etiam vel ipsum quis eros sollicitudin accumsan at eu ipsum. Maecenas sed maximus urna. Phasellus eget neque vel nisl finibus scelerisque sit amet eu nunc.

{{---}}

## Section 3

Quisque nec dapibus nisi. Aliquam vehicula consequat libero eu dapibus. Curabitur nec diam suscipit, fringilla ligula et, suscipit ipsum. Sed sodales eros quis maximus faucibus. Integer sit amet pulvinar nulla.
```

## Install

```shell
cargo install mdbook-pagebreaks
```

## Initialize

```shell
cd path/to/book
mdbook-pagebreaks init
```

## Configure

```toml
[preprocessor.pagebreaks]
[output.html]
additional-css = ["mdbook-pagebreaks.css"]
```
