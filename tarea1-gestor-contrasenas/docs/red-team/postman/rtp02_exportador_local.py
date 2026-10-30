"""Auxiliar de laboratorio: solo loopback, un evento ficticio, sin claves privadas.
Ejecutar con Python del runtime de Codex en el host Windows. Requiere SSH del laboratorio.
"""
import base64,json,subprocess,time
from http.server import BaseHTTPRequestHandler,HTTPServer
from pathlib import Path

PYTHON=Path('C:/Users/diego/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe')
SSH=Path('D:/Laboratorio-RedTeam/Linux-Backend/lab_ssh.py')
SQL="SELECT row_to_json(x) FROM (SELECT tipo,sistema,agente_id,to_char(occurred_at AT TIME ZONE 'UTC','YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') AS timestamp,firma_jws FROM audit_events WHERE sistema='RT-P02-Original' AND agente_id='agente-215b7431' ORDER BY id DESC LIMIT 1) x;"

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path!='/rt-p02/latest':return self.reply(404,{'error':'Ruta inexistente'})
        try:
            import shlex
            command='cd /opt/redteam-lab/infra && sudo docker compose exec -T postgres psql -U gestor -d control_central -At -c '+shlex.quote(SQL)
            result=subprocess.run([str(PYTHON),str(SSH),command],capture_output=True,text=True,encoding='utf-8',timeout=30)
            if result.returncode:return self.reply(503,{'error':'No se pudo consultar el backend por SSH. Revisar VM y configuración.'})
            lines=[line for line in result.stdout.splitlines() if line.startswith('{')]
            if not lines:return self.reply(404,{'error':'Crear desde el gestor un alta ficticia con sistema RT-P02-Original.'})
            body=json.loads(lines[-1]);token=body['firma_jws'];part=token.split('.')[1]
            claims=json.loads(base64.urlsafe_b64decode(part+'='*(-len(part)%4)))
            if claims.get('exp',0)<=time.time():return self.reply(409,{'error':'Firma vencida. Generar otro alta RT-P02-Original desde la app.'})
            if any(claims.get(k)!=body[k] for k in ['tipo','sistema','agente_id']) or claims.get('ts')!=body['timestamp']:
                return self.reply(409,{'error':'Campos y claims no coinciden. No utilizar este evento.'})
            altered={**body,'sistema':'RT-P02-Alterado'}
            self.reply(200,{'original':body,'alterado':altered,'expires_at_unix':claims['exp'],'nota':'Lectura PostgreSQL; no firma ni envía eventos. La API debe validar criptográficamente el original.'})
        except Exception:
            self.reply(503,{'error':'No se pudo exportar un evento vigente. Revisar laboratorio; no se enviaron eventos.'})
    def reply(self,status,body):
        data=json.dumps(body,ensure_ascii=False,indent=2).encode()
        self.send_response(status);self.send_header('Content-Type','application/json; charset=utf-8');self.send_header('Cache-Control','no-store');self.end_headers();self.wfile.write(data)
    def log_message(self,*args):pass

if __name__=='__main__':
    print('Exportador local: http://127.0.0.1:18002/rt-p02/latest — Ctrl+C para cerrar. Sin CORS ni acceso de red externo.')
    HTTPServer(('127.0.0.1',18002),Handler).serve_forever()
