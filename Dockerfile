FROM mariadb:latest as builder
RUN ["sed", "-i", "s/exec \"$@\"/echo \"not running $@\"/", "/usr/local/bin/docker-entrypoint.sh"]

ENV MYSQL_ROOT_PASSWORD=root
ENV MYSQL_USER=mysql
ENV MYSQL_ROOT_PASSWORD=vagrant

COPY TDB_full_world_335.20121_2020_12_15.sql.gz /docker-entrypoint-initdb.d/
RUN ["/usr/local/bin/docker-entrypoint.sh", "mysqld", "--datadir", "/initialized-db", "--aria-log-dir-path", "/initialized-db"]

FROM mariadb:latest
EXPOSE 3306

COPY --from=builder /initialized-db /var/lib/mysql