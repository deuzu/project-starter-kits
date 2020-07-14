FROM node:alpine

# npm loglevel in base image is verbose, adjust to warnings only
ENV NPM_CONFIG_LOGLEVEL warn

# Set environment variables for reveal.js GitHub repo
ENV VERSION=4.0.2 \
    REPO=https://github.com/hakimel/reveal.js
    # SHA1=b444529be8a1041942f3f30ae4f626e10497498c

# Install dependencies for fetching reveal.js source code
RUN set -ex \
    && apk add --no-cache --virtual .fetch-deps \
        ca-certificates \
        tar \
        wget

# Fetch reveal.js
RUN set -ex \
    && wget -qO /tmp/reveal.js.tar.gz $REPO/archive/$VERSION.tar.gz \
    # && echo "$SHA1  /tmp/reveal.js.tar.gz" | sha1sum -c - \
    && tar -xzf /tmp/reveal.js.tar.gz -C / \
    && rm -f /tmp/reveal.js.tar.gz \
    && mv reveal.js-$VERSION reveal.js

# Install build tools
RUN set -ex \
    && apk add --no-cache --virtual .build-deps \
        g++ \
        make \
        python

# Build and install dependencies
RUN set -ex \
    && mkdir -p /reveal.js/node_modules \
    && npm install -g gulp-cli \
    && npm install --prefix /reveal.js

# Clean up
RUN set -ex \
    && rm -rf /root/.node-gyp /tmp/npm* /tmp/phantomjs \
    && apk del .fetch-deps \
    && apk del .build-deps \
    && chown -R node:node /reveal.js

VOLUME /app
WORKDIR /app
USER node
EXPOSE 8000

COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
CMD ["npm", "start"]
