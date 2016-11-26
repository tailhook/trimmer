.. Trimmer documentation master file, created by
   sphinx-quickstart on Sat Nov 26 00:54:57 2016.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Welcome to Trimmer's documentation!
===================================

Trimmer is a template language that was initially created to make configuration
files.

Contents:

.. toctree::
   :maxdepth: 2


Quick showcase:

.. code-block:: bash

    ## syntax: indent, {}
    http {
        ## for server in servers
            ## skip if server.hostname == 'null'
            server {
                root /var/www;
                server_name
                    ## for name in server.hostnames
                       {{+ server.hostname }}.{{ suffix -}}
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
* :ref:`modindex`
* :ref:`search`

