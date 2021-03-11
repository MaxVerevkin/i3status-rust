# Formatting

## Syntax

The syntax for placeholders is

```
{<name>[:[0]<min width>][^<max width>][;<min prefix>][*<unit>][#<bar max value>]}
```

### `<name>`

This is just a name of a placeholder. Each block that uses formatting will list them under "Available Format Keys" section of their config.

### `[0]<min width>`

Sets the minimum width of the content (in characters). If starts with a zero, `0` symbol will be used to pad the content. A space is used otherwise. Floats and Integers are shifted to the right, while Strings are to the left.

#### Examples (□ is used instead of spaces)

`"{var:3}"`

The value of `var` | Output
-------------------|--------
`"abc"`            | `"abc"`
`"abcde"`          | `"abcde"`
`"ab"`             | `"ab□"`
`1`                | `"□□1"`
`1234`             | `"1234"`
`1.0`              | `"1.0"`
`12.0`             | `"□12"`
`123.0`            | `"123"`
`1234.0`           | `"1234"`

### `<max width>`

Sets the maximum width of the content (in characters). Applicable only for Strings. 

#### Examples

`"{var^3}"`

The value of `var` | Output
-------------------|--------
`"abc"`            | `"abc"`
`"abcde"`          | `"abc"`
`"ab"`             | `"ab"`

### `<min prefix>`

//FIXME

### `<unit>`

//FIXME

### `<bar max value>`

Every numeric placeholder (Integers and Floats) can be drawn as a bar. This option sets the value to be considered "100%". If this option is set, every other option will be ignored, except for `min width`, which will set the length of a bar.

#### Example

```toml
[[block]]
block = "sound"
format = "{volume:5#110} {volume:03}"
```

Here, `{volume:5#110}` means "draw a bar, 5 character long, with 100% being 110.

Output: https://imgur.com/a/CCNw04e
