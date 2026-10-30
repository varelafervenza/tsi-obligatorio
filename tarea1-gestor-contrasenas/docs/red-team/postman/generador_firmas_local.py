"""Firmador de fixtures de laboratorio, loopback. No envía eventos a la API."""
import json,os,sys,time
from datetime import datetime,timezone
from http.server import BaseHTTPRequestHandler,HTTPServer
from pathlib import Path
sys.path.insert(0,'D:/Laboratorio-RedTeam/Linux-Backend/tools')
from cryptography.hazmat.primitives import hashes,serialization
from cryptography.hazmat.primitives.asymmetric import rsa,padding
import base64

AGENT='agente-postman-pruebas'
DIRECTORY=Path(os.environ['LOCALAPPDATA'])/'TSI-Laboratorio'/'firmas-postman'
DIRECTORY.mkdir(parents=True,exist_ok=True)
KEY=DIRECTORY/'agente-postman-pruebas.pem'
PUBLIC=DIRECTORY/'agente-postman-pruebas.pub.pem'
if KEY.exists():private=serialization.load_pem_private_key(KEY.read_bytes(),password=None)
else:
    private=rsa.generate_private_key(public_exponent=65537,key_size=2048)
    KEY.write_bytes(private.private_bytes(serialization.Encoding.PEM,serialization.PrivateFormat.PKCS8,serialization.NoEncryption()))
PUBLIC.write_bytes(private.public_key().public_bytes(serialization.Encoding.PEM,serialization.PublicFormat.SubjectPublicKeyInfo))
def b64(data):return base64.urlsafe_b64encode(data).rstrip(b'=').decode()
TYPES={'alta_credencial','modificacion_credencial','borrado_credencial','cambio_maestra','intento_fallido_maestra','vencimiento_credencial'}
class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        if self.path!='/firmar':return self.reply(404,{'error':'Usar POST /firmar'})
        if self.headers.get('Origin') or self.headers.get('X-TSI-Lab')!='firmas-prueba' or self.headers.get('Content-Type','').split(';')[0]!='application/json':
            return self.reply(403,{'error':'Solo cliente local de laboratorio, sin Origin, JSON y cabecera X-TSI-Lab.'})
        try:
            size=int(self.headers.get('Content-Length','0'))
            if size<1 or size>4096:return self.reply(400,{'error':'Body vacío o demasiado grande'})
            data=json.loads(self.rfile.read(size))
            if set(data)-{'tipo','sistema'}:return self.reply(400,{'error':'Solo se aceptan tipo y sistema; agente, timestamp y exp los genera el auxiliar.'})
            kind=data['tipo'];system=data['sistema']
            if kind not in TYPES or not isinstance(system,str) or not system.startswith('RT-') or len(system)>256:
                return self.reply(400,{'error':'tipo permitido y sistema ficticio con prefijo RT- (máximo 256 caracteres).'})
            now=int(time.time());stamp=datetime.fromtimestamp(now,timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
            claims={'agente_id':AGENT,'tipo':kind,'sistema':system,'ts':stamp,'exp':now+300}
            encoded=b64(json.dumps({'alg':'RS256','typ':'JWT'},separators=(',',':')).encode())+'.'+b64(json.dumps(claims,separators=(',',':')).encode())
            token=encoded+'.'+b64(private.sign(encoded.encode(),padding.PKCS1v15(),hashes.SHA256()))
            self.reply(200,{'firma_jws':token,'body_evento':{'tipo':kind,'sistema':system,'agente_id':AGENT,'timestamp':stamp,'firma_jws':token},'expires_at_unix':now+300,'vigencia_segundos':300,'enviado_al_backend':False})
        except (ValueError,KeyError,TypeError):self.reply(400,{'error':'JSON inválido; indicar tipo y sistema'})
    def reply(self,code,body):
        raw=json.dumps(body,ensure_ascii=False,indent=2).encode()
        self.send_response(code);self.send_header('Content-Type','application/json');self.send_header('Cache-Control','no-store');self.end_headers();self.wfile.write(raw)
    def log_message(self,*args):pass
if __name__=='__main__':
    if '--init' in sys.argv:print(str(PUBLIC))
    else:HTTPServer(('127.0.0.1',18003),Handler).serve_forever()
