FROM postgres:15.2
COPY database_setup.sql /docker-entrypoint-initdb.d/database_setup.sql
# This container should only be visible from other containers, so no strong password is required
ENV POSTGRES_PASSWORD postgres_password