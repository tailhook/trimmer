Welcome to Trimmer's documentation!
===================================

Trimmer is a template language that was initially created to make configuration
files. Basically it's template renderer for text files which can also do
very small subset of validations to make process of design of template easier
and safer to use.

Github_ | `API Docs`_ | Crate_

.. _api docs: https://docs.rs/trimmer/
.. _github: https://github.com/tailhook/trimmer/
.. _crate: https://crates.io/crates/trimmer

Contents:

.. toctree::
   :maxdepth: 2

   template_syntax
   changelog

.. _showcase:

Quick showcase:

.. code-block:: bash

    ## syntax: indent
    ## validate default: [^;{}]*  # no ; { } to break nginx syntax
    ## validate ne: [^;{}]+       # non-empty (note plus in regex)
    http {
        ## for server in servers
            ## skip if server.hostname == ""
            server {
                root /var/www;
                server_name
                    ## for name in server.hostnames
                       {{+ server.hostname | ne }}.{{ suffix | ne -}}
                    ## endfor
                ;
                ## if server.ip
                    listen {{ server.ip }}:80;
                ## else
                    listen 80;
                ## endif
            }
        ## endfor
    }

Results in (note the indentation):

.. code-block:: nginx

    http {
        server {
            root /var/www;
            server_name apple.local;
            listen 192.168.0.1:80;
        }
        server {
            root /var/www;
            server_name orange.local plum.local;
            listen 80;
        }
    }

Since v0.3.6 it also works well for HTML and has autoescaping,
here is an example:

.. code-block:: html

    ## syntax: indent
    ## filter default: builtin.html_entities  ### autoescape
    ## validate n: [0-9]+
    <!DOCTYPE html>
    <html>
        <body>
            <h1>{{ title }}</h1>               ### auto escaped
            <div class="user_canvas"
                style="position: absolute;     ### note: it's unsafe to just
                       left: {{ x | n }}px;    ### autoescape values here
                       top: {{ y | n }}px;">
            </div>
        </body>
    </html>

While the validation here looks like excessive (you must prevalidate it in
the app) some time ago we thought that escaping values against XSS manually is
a normal practice, now we always use autoescape. We look at validation of other
values directly in the template as another way to minimize human error.


Indices and tables
==================

* :ref:`genindex`
* :ref:`search`

