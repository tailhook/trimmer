Welcome to Trimmer's documentation!
===================================

Trimmer is a template language that was initially created to make configuration
files. Basically it's template renderer for text files which can also do
very small subset of validations to make process of design of template easier
and safer to use.

Contents:

.. toctree::
   :maxdepth: 2


Quick showcase:

.. code-block:: bash

    ## syntax: indent, {}
    ## validate default: '^[^;{}]*$'  # no ; { } to break nginx syntax
    ## validate ne: '^[^;{}]+$'       # non-empty (note plus in regex)
    http {
        ## for server in servers
            ## skip if server.hostname == null
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


Indices and tables
==================

* :ref:`genindex`
* :ref:`search`

