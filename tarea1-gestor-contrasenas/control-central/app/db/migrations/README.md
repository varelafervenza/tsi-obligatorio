# migrations

El prototipo H2 crea tablas con `Base.metadata.create_all` al arrancar (`app/main.py`).
`alembic init` queda para cuando el esquema deje de cambiar en cada commit.
