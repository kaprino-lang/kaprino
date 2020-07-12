# Projects of kaprino

Tools or libraries which are produced by kaprino-lang are here.

![Library relationships](https://github.com/kaprino-lang/kaprino/blob/master/img/tool_description.svg)

## kast (Kaprino AST generator)

This library supports to generate AST(Abstruct Syntax Tree) from kaprino source files.

## kgen (Kaprino code generator)

This library supports to find source code errors and to convert AST to LLVM intermediate language.

## kpls (Kaprino language server protocol)

This is a lanugauge server protocol for kaprino.
With this feature, you can write kaprino code with syntax highlight.

## kprc (Kaprino compiler)

This is a tool which compile kaprino code into an executable file.
This depends on kast and kgen.

## stdlib (Kaprino standard library)

This is a standard library for kaprino.
