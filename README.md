# Kompilator

Kompilator prostego języka imperatywnego do kodu maszyny wirtualnej.

Napisany 100% w języku rust z użyciem parsera LALRPOP

System obsługujący kompilacje jest skomplikowany, składa się z kilku modułów, ale co najważniejsze działa.

Kompilator nie obsługuje zbyt wiele optymalizacji.

Przechodzi wszystkie testy.

Autor: Paweł Stanik 272338

Uruchamianie:

1. Trzeba zainstalować język rust, można użyć polecenia
   ```
   make req
   ```
2. Kompilujemy kompilator poleceniem
   ```
   make
   ```
   lub
   ```
   cargo build --release
   ```
4. Plik wynikowy znajduje się w kataloogu
   ```
   target/release/
   ```
5. Kompilator uruchamiamy poleceniem
   ```
   kompilator input.imp output.mr
   ```
