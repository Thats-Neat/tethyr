FROM debian:bookworm

RUN mkdir /home/workspace
WORKDIR /home/workspace

RUN apt-get update && apt-get install -y \
    git \
    vim \
    curl \
    build-essential

COPY environment/base/entrypoint.sh /entrypoint.sh
COPY . .
RUN chmod +x /entrypoint.sh

ENTRYPOINT [ "/entrypoint.sh" ]
CMD [ "/bin/bash" ]