Operation Table Generator
=========================

## Usages

- Generate base 12 multiplication table

```rust
println!("$$");
let m = OperationTable::default().with_base(12);
println!("{}", m.display());
println!("$$");
```

$$
\begin{array}{|c|c|c|c|c|c|c|c|c|c|c|}
\hline
×_{(10)}&2&3&4&5&6&7&8&9&10&11\\
\hline
2&4&&&&&&&&&\\
\hline
3&6&9&&&&&&&&\\
\hline
4&8&12&16&&&&&&&\\
\hline
5&10&15&20&25&&&&&&\\
\hline
6&12&18&24&30&36&&&&&\\
\hline
7&14&21&28&35&42&49&&&&\\
\hline
8&16&24&32&40&48&56&64&&&\\
\hline
9&18&27&36&45&54&63&72&81&&\\
\hline
10&20&30&40&50&60&70&80&90&100&\\
\hline
11&22&33&44&55&66&77&88&99&110&121\\
\hline
\end{array}
$$

- Generate a multiplication table in base 12 and express it in base 12

```rust
println!("$$");
let m = OperationTable::default()
    .with_base(12)
    .with_display(12);
println!("{}", m.display());
println!("$$");
```

$$
\begin{array}{|c|c|c|c|c|c|c|c|c|c|c|}
\hline
×_{(12)}&2&3&4&5&6&7&8&9&10&11\\
\hline
2&4&&&&&&&&&\\
\hline
3&6&9&&&&&&&&\\
\hline
4&8&10&14&&&&&&&\\
\hline
5&a&13&18&21&&&&&&\\
\hline
6&10&16&20&26&30&&&&&\\
\hline
7&12&19&24&2b&36&41&&&&\\
\hline
8&14&20&28&34&40&48&54&&&\\
\hline
9&16&23&30&39&46&53&60&69&&\\
\hline
10&18&26&34&42&50&5a&68&76&84&\\
\hline
11&1a&29&38&47&56&65&74&83&92&a1\\
\hline
\end{array}
$$

- Generate a multiplication table in base 12 and express it in base 12. Do not skip the symmetrical part.

```rust
println!("$$");
let m = OperationTable::default()
    .with_base(12)
    .with_display(12)
    .with_upper_triangle(true);
println!("{}", m.display());
println!("$$");
```

$$
\begin{array}{|c|c|c|c|c|c|c|c|c|c|c|}
\hline
×_{(12)}&2&3&4&5&6&7&8&9&10&11\\
\hline
2&4&6&8&a&10&12&14&16&18&1a\\
\hline
3&6&9&10&13&16&19&20&23&26&29\\
\hline
4&8&10&14&18&20&24&28&30&34&38\\
\hline
5&a&13&18&21&26&2b&34&39&42&47\\
\hline
6&10&16&20&26&30&36&40&46&50&56\\
\hline
7&12&19&24&2b&36&41&48&53&5a&65\\
\hline
8&14&20&28&34&40&48&54&60&68&74\\
\hline
9&16&23&30&39&46&53&60&69&76&83\\
\hline
10&18&26&34&42&50&5a&68&76&84&92\\
\hline
11&1a&29&38&47&56&65&74&83&92&a1\\
\hline
\end{array}
$$