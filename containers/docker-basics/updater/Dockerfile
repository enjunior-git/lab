FROM ubuntu:24.04 AS builder

WORKDIR /build
COPY updater.sh .
RUN chmod +x updater.sh

FROM alpine:3.21

RUN apk add --no-cache curl ca-certificates bash

RUN addgroup -g 101 appgroup && \
    adduser -u 101 -G appgroup -D appuser

WORKDIR /app

COPY --from=builder --chown=appuser:appgroup /build/updater.sh .

USER appuser
CMD ["./updater.sh"]
