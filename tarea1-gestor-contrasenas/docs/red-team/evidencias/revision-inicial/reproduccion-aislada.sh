set -eu
docker exec -i infra-control-central-1 python - <<'PY'
import json,urllib.request,urllib.error
from datetime import datetime,timezone
from sqlalchemy import create_engine,select
from sqlalchemy.orm import Session
from app.db.base import Base
from app.models.event import AuditEvent
from app.models.alert import Alert
from app.siem.reglas import evaluar_reglas
from app.notify.mailer import TIPOS_CON_CORREO
from app.main import app
engine=create_engine('sqlite:///:memory:')
Base.metadata.create_all(engine)
with Session(engine) as db:
 e=AuditEvent(agente_id='diagnostico-firma-invalida',tipo='cambio_maestra',sistema='Prueba-aislada',occurred_at=datetime.now(timezone.utc),received_at=datetime.now(timezone.utc),firma_jws='firma-invalida-de-prueba',firma_valida=False,ip_origen='127.0.0.1')
 db.add(e);db.commit();db.refresh(e)
 evaluar_reglas(db,e)
 alerts=list(db.scalars(select(Alert)))
 result={'prueba':'Evento con firma invalida en SQLite temporal en memoria','event_firma_valida':e.firma_valida,'alertas_generadas':[{'regla':a.regla,'evento_id':a.evento_id} for a in alerts],'tipo_incluido_en_notificaciones':e.tipo in TIPOS_CON_CORREO,'base_real_modificada':False,'correo_enviado':False}
 result['consultas_sin_authorization']={}
 for path in ['/api/events/?limit=1','/api/dashboard/kpis','/api/users/']:
  try:
   with urllib.request.urlopen('http://127.0.0.1:8000'+path,timeout=15) as response:
    content=json.loads(response.read());result['consultas_sin_authorization'][path]={'http':response.status,'tipo_respuesta':type(content).__name__}
    if isinstance(content,list):result['consultas_sin_authorization'][path]['registros']=len(content)
  except urllib.error.HTTPError as error:result['consultas_sin_authorization'][path]={'http':error.code}
 print(json.dumps(result,ensure_ascii=False))
PY
