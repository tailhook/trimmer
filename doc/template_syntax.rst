.. _template-syntax:

===============
Template Syntax
===============

A trimmer template is a simple text file, similarly to many template engines
out there. There are few kinds of delimiters supported in template:

* ``{{ a.b }}`` prints expression (usually just variable substitution) to the
  template output
* ``## something`` statements, they usually they last to the end of line

.. index:: Expressions

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


.. index:: Statements

Statements
==========

Statements start with double-sharp and a keyword. Here are examples:

* ``## syntax: indent``
* ``## if x > 0``
* ``## for a in array``

They must start at the start of the line, not counting the whitespace.

The ``syntax`` and ``validate`` statements must also be at the start of the
file and in the first column of the row.

As a special case empty statement ``##`` at the end of the line strips newline
and leading spaces on the new line, effectively joining two lines::

  function(  ##
    arg1,    ##
    arg2)

Would output ``function(arg1, arg2)``. This is useful if the whole output
line doesn't fit a template line. While you could use ``{{- '' +}}`` at
the same place, the line joiner ``##`` is simpler and more clear.


.. index:: pair: Syntax; Statement

Syntax Statement
================

Syntax statement looks like this::

    ## syntax: indent

Syntax doesn't influence parsing the template but the typechecking and the
output of the template.

Syntax statement must come the first in the file

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

.. _validate:
.. index:: pair: Validate; Statement

Validate Statement
==================

The validate statement is the core thing for producing valid template output.
By default template output is not validated. But if you add the following
to the beginning of the file::

    ## validate default: [a-z]+

The output of any variable can consist only of alphanumeric characters.
Validator is a regular expression, the ``^`` and ``$`` anchors are added
automatically.

The ``default`` validator is used for every expression that doesn't override
the validator. You can add a validator with any other name to be used in
code that possibly extends default syntax, for example::

    ## validate default: [a-zA-Z0-9]+
    ## validate quoted: [^']*
    #!/bin/sh
    echo {{ arg1 }} '{{ arg2 | quoted }}'

Here we generate a shell script. To be careful, we assume that it's only safe
to put alphanumeric characters into the file. But in single-quoted strings its
safe to put anything except a quote, so for all variables printed in quotes we
can add a ``quoted`` validator. See :ref:`front page <showcase>` for more
practical example.


.. index:: pair: If; Statement

If Statement
============

Conditional statement looks like::

    ## if something
        output something
    ## endif

In any case lines containing ``## if`` and ``## endif`` do not put into output.
In ``indent`` syntax the inner indentation of the block is also stripped.


.. index:: pair: For; Statement

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
output.  In ``indent`` syntax the inner indentation of the block is also
stripped.

For loop does **not** support imperative ``break`` and ``continue`` statements,
but it allows filtering values by using ``## skip if``::

    ## for key, value in var
        ## skip if key == "bad_value"
        {{ key }} = {{ value }}
    ## endfor

This is works just like the following, but allows to keep indentation lower::

    ## for key, value in var
        ## if key != "bad_value"
            {{ key }} = {{ value }}
        ## endif
    ## endfor

