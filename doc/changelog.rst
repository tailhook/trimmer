==========================
Trimmer Changes By Version
==========================


.. _changelog-v0.3.6:

v0.3.6
======

* feature: Add python-like dictionary syntax support: ``{"a": 1}``. These
  dictionaries can be used instead of multiple ``if`` statements or to refer
  to the same values multiple times
* feature: Add python list syntax support: ``["item1", "item2"]``
* feature: Add support for filters, in particular ``builtin.html_entities``
  and ``builtin.quoted_shell_argument``


.. _changelog-v0.3.5:

v0.3.5
======

* Validate directives with some regexes was broken from the start and more of
  that was broken in v0.3.4, fixed now
* Add ``skip if`` satement to the :ref:`for loop <for-statement>`


.. _changelog-v0.3.4:

v0.3.4
======

* Adds line joiner ``##`` (at end of line) syntax
* Bugfixes of syntax and command-line tool


.. _changelog-v0.3.3:

v0.3.3
======

* Implemented all comparison operators (``>, <, >=, <=, ==, !=``)
* Implemented ``as_comparable()`` for ``serde_json::Value``
* Added parenthesis support in expressions


.. _changelog-v0.3.2:

v0.3.2
======

* Implemented ``as_number()`` for ``serde_json::Value``
* Implemented ``a*b``, ``a/b`` and ``a%b`` expressions
* [bugfix] Previously ``a.x+b`` worked but ``a.x +b`` did not (whitespace bug)


.. _changelog-v0.3.1:

v0.3.1
======

* ``as_number()`` and ``as_comparable()`` methods added to the ``Variable``
  trait
* Implemented ``a+b`` and ``a-b`` syntax in templates
* ``## validate`` statements implemented (See :ref:`documentation <validate>`)
* The ``# comments`` inside line statements are supported now
* The ``### comments`` can be used as line comments in normal context
* Added ``--version`` parameter to command-line tool
