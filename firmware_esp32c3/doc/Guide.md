# Dokumentation: Wasser-Drohne mit ESP32 und CAN-Bus

## 1. Systemarchitektur

Das System ist dezentral aufgebaut und trennt die Videoübertragung physisch von
der Steuerungs- und Telemetriedatenübertragung.

| Komponente            | Mikrocontroller | Aufgabe                                      | Datenverbindung          |
| :-------------------- | :-------------- | :------------------------------------------- | :----------------------- |
| **Fernbedienung**     | ESP32           | Auslesen der Joysticks, Senden an TX-Modul   | CRSF (UART) an ELRS-TX   |
| **Zentrale (Master)** | ESP32           | Empfang der RC-Signale, Weiterleitung an CAN | CRSF (UART) / CAN-Bus    |
| **Motor-Module (4x)** | ESP32           | Empfang der Soll-Werte, Motorsteuerung (PWM) | CAN-Bus                  |
| **Video-System**      | Autark          | Eigenständige 1080p-Kamera und Übertragung   | 5.8 GHz Digital (Autark) |

---

## 2. Hardware-Liste

### Zentrale Steuerung & Motoren

- **5x ESP32-Entwicklungsboards:** (1x Master, 4x Motoren).
- **5x CAN-Transceiver (SN65HVD230):** Für die 3,3V-Logik der ESP32-Chips.
- **4x Motorregler (ESCs) & Elektromotoren:** Passend zur gewünschten Leistung.
- **Spannungswandler (Step-Down/BEC):** Zur Versorgung der ESP32 (5V oder 3,3V)
  aus dem Hauptakku (z.B. 12V/24V).

### Funkverbindung (Steuerung)

- **ELRS TX-Modul (Fernbedienung):** 2,4 GHz (z.B. Radiomaster Micro).
- **ELRS Nano-Empfänger (Boot):** 2,4 GHz mit T-Antenne (Reichweite ca. 2–5 km
  auf dem Wasser).
- **Joysticks (Fernbedienung):** 2x analoge Hall-Sensor-Gimbals.

### Funkverbindung (Video)

- **Digitales FPV-Set:** Walksnail Avatar HD, DJI O3 Air Unit oder HDZero.
- **Empfänger:** Kompatible FPV-Brille oder HDMI-Bodenstations-Empfänger.

---

## 3. Verkabelung und Netzwerk (CAN-Bus)

Der CAN-Bus dient der fehlerresistenten Kommunikation zwischen dem Master und
den 4 Motor-Modulen.

- **Topologie:** Linien-Topologie (Strang). Alle Module werden an eine
  gemeinsame CAN-High und CAN-Low Leitung angeschlossen.
- **Transceiver:** Der ESP32 verbindet seine Pins (z.B. GPIO4/GPIO5) mit den
  RX/TX-Pins des SN65HVD230. Dieser übersetzt das Signal auf den Bus.
- **Terminierung (WICHTIG):** Genau die beiden physischen Enden des Bus-Kabels
  müssen mit einem **120-Ohm-Abschlusswiderstand** versehen werden. Bei den drei
  Modulen in der Mitte des Strangs muss dieser Widerstand auf dem
  Transceiver-Board entfernt/ausgelötet werden.
- **Adressierung (CAN-IDs):**
  - Motor 1: `0x101`
  - Motor 2: `0x102`
  - Motor 3: `0x103`
  - Motor 4: `0x104`

---

## 4. Software-Architektur (Rust & esp-idf-hal)

Alle ESP32-Komponenten nutzen das `esp-idf-hal` Crate unter FreeRTOS.

### Fernbedienung (Joystick -> CRSF)

1.  **ADC-Pins** auslesen, um Joystick-Positionen (0-4095) zu erfassen.
2.  Werte auf den **CRSF-Wertebereich** (172–1811) mappen.
3.  Ein 26-Byte großes CRSF-Paket (inkl. CRC8-Prüfsumme) zusammenstellen.
4.  Paket per **UART (400.000 Baud)** an das ELRS-Sendemodul übertragen.

### Master (CRSF -> CAN-Bus)

1.  **UART-Empfang** (400.000 Baud) zum Auslesen des ELRS-Empfängers einrichten.
2.  **CAN-Treiber (`CanDriver`)** mit 500 kbit/s initialisieren.
3.  CRSF-Pakete dekodieren, Geschwindigkeitswerte für die 4 Motoren extrahieren.
4.  Werte als 4-Byte (Little-Endian) `u32` formatiert per
    `can_driver.transmit()` an die jeweiligen CAN-IDs senden.

### Motor-Module (CAN-Bus -> PWM)

1.  **CAN-Treiber** initialisieren.
2.  Hardware-Filter oder Software-Abfragen nutzen, um nur auf die eigene CAN-ID
    (z.B. `0x101`) zu reagieren.
3.  Daten in blockierender Schleife empfangen:
    `can_driver.receive(esp_idf_svc::hal::delay::BLOCK)`.
4.  Den empfangenen 4-Byte-Payload in einen `u32` (Prozentwert) umwandeln.
5.  Die bestehende Methode `set_motor_speed(speed)` aufrufen, um das
    PWM/LEDC-Signal an den Motorregler (ESC) anzupassen.
