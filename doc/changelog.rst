==========================
Trimmer Changes By Version
==========================

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

.. _changelog-v0.3.2:

v0.3.2
======

* Implemented ``as_number()`` for ``serde_json::Value``
* Implemented ``a*b``, ``a/b`` and ``a%b`` expressions
* [bugfix] Previously ``a.x+b`` worked but ``a.x +b`` did not (whitespace bug)
