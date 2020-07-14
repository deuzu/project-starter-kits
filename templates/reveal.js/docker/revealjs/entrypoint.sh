#!/bin/sh

test -s /app/package.json || (mv /reveal.js/* /app/ && mv /reveal.js/.* /app/)
npm install

exec "$@"
