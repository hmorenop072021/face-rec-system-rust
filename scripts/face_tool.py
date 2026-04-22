import os
import requests
import argparse
from pathlib import Path
import json

# --- CONFIGURACIÓN ---
API_BASE_URL = "http://localhost:3000"

def enroll(user_id, image_path):
    url = f"{API_BASE_URL}/enroll"
    with open(image_path, 'rb') as f:
        files = {
            'user_id': (None, user_id),
            'image': (os.path.basename(image_path), f, 'image/jpeg')
        }
        response = requests.post(url, files=files)
        print(json.dumps(response.json(), indent=2))

def identify(image_path):
    url = f"{API_BASE_URL}/identify"
    print(f"🔍 Identificando rostro en: {image_path}...")
    with open(image_path, 'rb') as f:
        files = {
            'image': (os.path.basename(image_path), f, 'image/jpeg')
        }
        response = requests.post(url, files=files)
        
        if response.status_code == 200:
            result = response.json()
            if result.get("status") == "found":
                print(f"✅ EMPLEADO IDENTIFICADO: {result['user_id']}")
                print(f"📊 Puntaje de Similitud: {result['score']:.4f}")
            else:
                print("❌ Rostro no reconocido (Bajo el umbral o no enrolado).")
        else:
            print(f"🔥 Error del servidor: {response.text}")

def main():
    parser = argparse.ArgumentParser(description="Face Recognition Toolkit")
    subparsers = parser.add_subparsers(dest="command", help="Comandos disponibles")

    # Comando: Enroll
    enroll_parser = subparsers.add_parser("enroll", help="Enrolar un nuevo empleado")
    enroll_parser.add_argument("--id", required=True, help="RUT o ID del empleado")
    enroll_parser.add_argument("--image", required=True, help="Ruta a la foto")

    # Comando: Identify
    identify_parser = subparsers.add_parser("identify", help="Identificar a una persona (1:N)")
    identify_parser.add_argument("--image", required=True, help="Ruta a la foto de prueba")

    args = parser.parse_args()

    if args.command == "enroll":
        enroll(args.id, args.image)
    elif args.command == "identify":
        identify(args.image)
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
