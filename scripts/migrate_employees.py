import os
import requests
import argparse
from pathlib import Path

# --- CONFIGURACIÓN ---
# La URL de la API de Rust corriendo en Docker (face-api en docker-compose)
DEFAULT_API_URL = "http://localhost:3000/enroll"

def migrate_employees(base_dir, api_url):
    """
    Recorre una estructura de carpetas donde cada carpeta es un RUT (o ID de empleado)
    y contiene sus fotos, enviándolas al sistema de reconocimiento facial en Rust.
    """
    base_path = Path(base_dir)
    
    if not base_path.exists():
        print(f"❌ Error: La ruta {base_dir} no existe.")
        return

    print(f"🚀 Iniciando migración masiva desde: {base_path.absolute()}")
    print(f"🔗 Conectando con API: {api_url}")
    print("-" * 50)

    # Estadísticas
    success_count = 0
    error_count = 0

    # Iterar sobre cada subcarpeta (nombre de la carpeta = RUT)
    for user_dir in base_path.iterdir():
        if not user_dir.is_dir():
            continue
            
        rut = user_dir.name
        
        # Buscar la primera imagen válida (jpg, jpeg, png)
        valid_extensions = ('.jpg', '.jpeg', '.png')
        photos = [f for f in user_dir.iterdir() if f.suffix.lower() in valid_extensions]
        
        if not photos:
            print(f"⚠️  [RUT: {rut}] No se encontraron fotos válidas en la carpeta.")
            continue
            
        # Tomamos la primera foto para el enrolamiento inicial (ArcFace es One-Shot)
        primary_photo = photos[0]
        
        print(f"📤 Enrolando RUT: {rut} (Archivo: {primary_photo.name})...", end=" ", flush=True)
        
        try:
            with open(primary_photo, 'rb') as f:
                # El contrato multipart esperado por nuestro servidor Axum en Rust
                files = {
                    'user_id': (None, rut),
                    'image': (primary_photo.name, f, f'image/{primary_photo.suffix[1:]}')
                }
                
                response = requests.post(api_url, files=files)
                
                if response.status_code == 200:
                    print("✅ ÉXITO")
                    success_count += 1
                else:
                    print(f"❌ ERROR ({response.status_code}): {response.text}")
                    error_count += 1
                    
        except Exception as e:
            print(f"🔥 FALLO DE CONEXIÓN: {e}")
            error_count += 1

    print("-" * 50)
    print(f"📊 Resumen de Migración:")
    print(f"   - Empleados enrolados con éxito: {success_count}")
    print(f"   - Errores encontrados: {error_count}")
    print(f"   - Total procesados: {success_count + error_count}")
    print("-" * 50)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Script de migración masiva de empleados a Face Recognition Rust")
    parser.add_argument("dir", help="Directorio base que contiene las carpetas de los empleados (RUT)")
    parser.add_argument("--url", default=DEFAULT_API_URL, help=f"URL del endpoint /enroll (Default: {DEFAULT_API_URL})")
    
    args = parser.parse_args()
    migrate_employees(args.dir, args.url)
