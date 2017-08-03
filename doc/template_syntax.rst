.. _template-syntax:

===============
Template Syntax
===============

A trimmer template is a simple text file, similarly to many template engines
out there. There are few kinds of delimiters supported in template:

* ``{{ a.b }}`` prints expression (usually just variable substitution) to the
  template output
* ``## something`` statements, they usually they last to the end of line


Expressions
===========

Simple expression substitutes a variable passed to template context of
a renderer::

    {{ var }}

You can use attribute access via dot ``a.b`` and item access via brackets
``a["b"]``. It depends on the type of variable (i.e. the implementor of
``Variable`` trait) whether this means the same or a different thing. For
most types these kind of accesses are equivalent.

There are also whitespace control flags on an expression. If there is a
dash ``-`` char on either end of the expression, then all of the adjacent
whitespace is stripped, for example, the following examples will render
the same::

    {{ x }}{{ y }}
    {{ x -}}     {{ y }}
    {{ x }}   {{- y }}
    {{ x -}}    {{- y }}

This works even if expressions are divided by newlines, comments, loops and
conditional staments.

Another mode of whitespace control is using plus ``+`` character instead, this
strips all the whitespace and puts exactly one space between variables. These
examples are all equivalent::

    {{ x }} {{ y }}
    {{ x +}}{{+ y }}
    {{ x +}}    {{ y }}
    {{ x }}    {{+ y }}

In case of conflict (i.e. one operator on the left and other on the right):

* Plus ``+`` operator overrides no operator at all
* Dash ``-`` operator overrides both plus ``+`` and no operator

Additional notes:

1. The plus operator doesn't add space at the beginning and the
   end of the buffer
2. We don't strip spaces output by an expression, only template source

The rule (2) in particular means, you can protect spaces using expression, in
particular these are equivalent::

   {{ x }}   {{ y }}
   {{ x +}}    {{ " " }}    {{+ y }}
   {{ x -}}   {{ "   " }}   {{- y }}
   {{ x -}}   {{ "" }}   {{ "" }}   {{- y }}

*(yes even empty string is a delimiter that stops whitespace removal)*

As you have already seen you can also put a bare string in expression, this is
the easiest way to put expression or statement delimiter into the file::

    {{ '{{' }}


Statements
==========

Statements start with double-sharp and a keyword. Here are examples:

* ``## syntax: indent``
* ``## if x > 0``
* ``## for a in array``

They must start at the start of the line, not counting the whitespace.


Syntax Statement
================

Syntax statement looks like this::

    ## syntax: indent

Syntax doesn't influence parsing the template but the typechecking and the
output of the template.

Here are the list of supported syntaxes:

.. _syntax-indent:
.. describe:: ## syntax: indent

  Means the output is indentation-sensitive, and
  we strip the additional indentation created by the block statements
  (``## if`` / ``## endif`` and ``## for`` / ``## endfor``), so the
  final output can easily be YAML or Python code.

.. _syntax-oneline:
.. describe:: ## syntax: oneline

  All subsequent whitespace (including newlines)
  is condensed and treated as a single space, effectively making template
  a oneline thing. This syntax is useful for templating log formats
  and command-lines.

  Note: all whitespace printed by expressions is preserved, so you might
  escape whitespace and newlines using quoted literals (``{{ "\n" }}``),
  unless they are rejected by a validator.

.. describe:: <plain-syntax>

  Plain (no syntax statement) means the output of the template is rendered
  as is with all whitespace. Statements always occupy the whole line
  including indentation whitespace and trailing end of line.


If Statement
============

Conditional statement looks like::

    ## if something
        output something
    ## endif

In any case lines containing ``## if`` and ``## endif`` do not put into output.
In ``indent`` syntax the inner indentation of the block is also stripped.


For Statement
=============

There are two forms of loop statements, for iterating over sequences::

    ## for var in value
        output something
    ## endfor

And for iterating over dictionaries::

    ## for key, value in var
        {{ key }} = {{ value }}
    ## endfor

In any case lines containing ``## for`` and ``## endfor`` do not put into
output.  In ``indent`` syntax the inner indentation of the block is also stripped.
