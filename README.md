# BTree database
## Instruction
### Polish
#### Uruchomienie programu w wersji interaktywnej:
```cargo run <file_path> <order> <print_opt> <buffer_size> <cache_size>```


#### Uruchomienie programu w wersji czytającej z pliku:
```
cargo run <file_path> <order> <print_opt> <buffer_size> <cache_size> < <instruction_file>
// lub opcja bez ‘<’
./read_from_file.sh <file_path> <order> <print_opt> <buffer_size> <cache_size> <instruction_file>
```

#### Wygenerowanie pliku testowego (z instrukcjami):
```cargo run <file_path> gen <instruction_count>```

#### Wyszukanie rekordu
```s <key>```


Wyszukanie rekordu polega na rekursywnym przejściu drzewa od korzenia. Jeżeli na danej stronie znajdzie się klucz to zwrócony jest Ok() zawierający ten rekord wraz z id strony, na której został znaleziony. W przeciwnym wypadku próba znalezienia rekordu jest ponowiona na odpowiednim jego dziecku. Jeżeli dana strona jest liściem to Zwracany jest Err() zawierający id strony oraz pozycję w tablicy rekordów na tej stronie gdzie możliwe jest wstawienie rekordu o danym kluczu.
#### Wstawienie rekordu
```i <key> <data>```


Wstawienie rekordu polega na znalezieniu odpowiedniej dla niego pozycji poprzez jego wyszukanie. Następnie jest on wstawiony na stronę. Jeżeli ilość rekordów na stronie przekroczy 2 * d, to najpierw zostanie podjęta próba kompensacji. Jeżeli takowa nie jest możliwa, to nastąpi rozdzielenie strony. Jeżeli będzie potrzebne więcej rozdziałów, to zostaną one wykonane rekursywnie.

Próba wstawienie rekordu, którego klucz już znajduje się w drzewie zostanie zablokowana.
#### Usunięcie rekordu
```d <key>```


Usunięcie rekordu polega na znalezieniu go, podmienieniu z odpowiednim rekordem z liścia w przypadku gdy nie jest on na liściu, a następnie usunięciu z drzewa. Następnie sprawdzane jest czy strona nadal spełnia warunek posiadania co najmniej d rekordów. Jeżeli nie to zostaje podjęta próba kompensacji. Jeżeli nie jest możliwa, to strona scali się z jedną ze swojego rodzeństwa. 

Usunięcie nieistniejącego rekordu nic nie robi.
#### Aktualizacja rekordu
```u <old_key> <new_key> <opt: new_data>```


Aktualizacja ma dwa warianty.

W przypadku, gdy stary i nowy klucz są takie same (rekord nie zmieni swojej pozycji w drzewie) jedyna akcja jaka zostanie wykonana to nadpisanie danych w pliku danych.

Jeżeli klucze się różnią to rekord zostanie najpierw usunięty, a potem dodany na nowo. W przypadku gdy nowe dane nie zostaną podane nie zostanie wykonany zapis do pliku danych. W przeciwnym wypadku dane zostaną nadpisane na swoim starym miejscu.

Aktualizacja nie nastąpi gdy: nie zostanie znaleziony rekord o podanym starym kluczu, zostanie znaleziony rekord o podanym nowym kluczu, który różni się od starego.
