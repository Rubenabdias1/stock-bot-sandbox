#!/bin/bash

chown -R rstudio:rstudio /home/rstudio
chown -R rstudio:rstudio /usr/local/lib/R/site-library

# Call the original entrypoint.
exec "/init"
