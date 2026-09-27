"""Configuración vía variables de entorno (ver infra/control-central.env.example)."""
from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    database_url: str = "postgresql://gestor:changeme@localhost:5432/control_central"
    jwt_public_keys_dir: str = "./keys/agentes"
    smtp_host: str = "mailpit"
    smtp_port: int = 1025
    smtp_from: str = "noreply@correo.local"
    smtp_to: str = "rsi@correo.local"
    smtp_user: str = ""
    smtp_password: str = ""
    smtp_use_tls: bool = False
    hash_algorithm: str = "argon2id"  # o "bcrypt", elegible desde RF-11
    siem_log_path: str = "./logs/audit-events.jsonl"

    class Config:
        env_file = ".env"


settings = Settings()
