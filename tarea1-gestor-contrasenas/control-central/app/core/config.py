"""Configuración vía variables de entorno (ver infra/control-central.env.example)."""
from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    database_url: str = "postgresql://gestor:changeme@localhost:5432/control_central"
    jwt_public_keys_dir: str = "./keys/agentes"
    smtp_host: str = "mailu"
    smtp_port: int = 587
    hash_algorithm: str = "argon2id"  # o "bcrypt", elegible desde RF-11

    class Config:
        env_file = ".env"


settings = Settings()
