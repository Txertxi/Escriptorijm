# Calendari i Rellotges (Windows)

Giny d'escriptori: data gran de plaques mecàniques, calendari mensual amb els festius de
**Valls**, **Querétaro** i **Myjava**, i tres rellotges digitals mecànics (24 h).
Finestra sense vora, ajustada al contingut, al fons de l'escriptori, arrossegable,
escalable i amb la posició recordada.

## Com obtenir l'.exe (GitHub)

1. Crea un repositori nou i puja-hi el contingut de la carpeta (arrossegant a la web).
2. **La carpeta `.github` no s'hi puja arrossegant.** Crea el workflow a mà:
   *Add file → Create new file*, escriu `.github/workflows/build.yml` al nom i enganxa-hi
   el contingut del fitxer `WORKFLOW-build.yml` d'aquesta carpeta.
3. Pestanya **Actions** → s'executa sol (o *Run workflow*). La primera vegada triga ~10 min.
4. Baixa l'artefacte **calendari-fusos-windows**: dins hi ha `calendari-fusos.exe`.
5. Posa l'.exe en una carpeta fixa (p. ex. `C:\Giny\`) i fes-hi doble clic.

## Autoinici amb Windows

La primera vegada que s'executa, el giny s'apunta sol a l'inici de Windows (clau
`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`). No cal cap drecera a `shell:startup`.

- Es pot desactivar a clic dret → Configuració → *Inicia amb Windows*. Si el desactives,
  no es torna a activar sol.
- Cada cop que s'obre amb l'autoinici activat, reescriu l'entrada amb la ruta actual de
  l'.exe (si el mous de carpeta, s'arregla amb una sola execució manual).
- Si s'intenta obrir una segona còpia, no se'n crea cap.

## Ús

- **Arrossegar:** clic esquerre en una zona buida.
- **Mida:** Ctrl + roda del ratolí, o el control a Configuració (50 %–150 %).
- **Clic dret:** Configuració, Sempre al fons, Tanca el giny.
- **Calendari:** ‹ › canvien de mes, *avui* hi torna. Passa el ratolí per sobre d'un dia
  festiu per veure el nom i a quina població.

## Festius (a `dist/index.html`, constant `HOL`)

- Cada població té un color: Valls groc, Querétaro verd, Myjava blau.
- Si un dia és festiu a diverses poblacions, el cercle es parteix en sectors.
- **Valls:** estatals + Catalunya + locals. Locals 2026: 2 de febrer i 25 de juny (ordre de
  festes locals 2026). **Locals 2027: provisionals** fins que es publiqui l'ordre (cap al
  desembre de 2026).
- **Querétaro:** dies de descans obligatori de la Ley Federal del Trabajo (art. 74), més
  Dijous/Divendres Sant, 15 de setembre i 2 de novembre (inhàbils al Poder Judicial de
  Querétaro; no obligatoris per llei). Els de 2027 d'aquest segon grup són suposició
  per analogia amb el 2026.
- **Myjava:** festius nacionals d'Eslovaquia que són dia de descans. El 8 de maig i el
  15 de setembre són laborables el 2026 i tornen a ser festiu el 2027. 1 de setembre,
  28 d'octubre i 17 de novembre són festius però laborables: no hi surten.
- Per afegir anys: copia un bloc, canvia les dates i afegeix l'any a `YEARS`.

## Estructura

```
dist/index.html         la pàgina (plaques mecàniques + calendari + rellotges)
src-tauri/              l'embolcall Tauri v2 (Rust)
  src/main.rs           posició, autoinici automàtic, còpia única
  capabilities/         permisos
icon.png                icona font; el workflow en genera totes les mides
WORKFLOW-build.yml      còpia del workflow per enganxar a la web de GitHub
```
