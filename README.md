# Pražák na výletě
Úkol Beta4 pro předmět PV na SPŠE Ječná

## Problém
Program řeší několika způsoby problém obchodního cestujícího(dále jen TSP). Tento problém je NP-Hard, takže najít jeho optimální řešení je exponenciální časová náročnost.

Toto můžeme potvrdit u `verze1`, která je jednoduchá implementace jedno-vláknového brute force algoritmu. 

| počet vrcholů | čas     |
| ------------- | ------- |
| 3             | 0ms     |
| 4             | 0ms     |
| 5             | 0ms     |
| 6             | 0ms     |
| 7             | 0ms     |
| 8             | 0ms     |
| 9             | 4ms     |
| 10            | 41ms    |
| 11            | 450ms   |
| 12            | 5400ms  |
| 13            | 69000ms |

*bere se pokaždé průměr z 10 běhů programu na různých náhodně vygenerovaných grafech, všechny grafy byly vytvořeny pomocí prográmku `graph_maker`. Ten naleznete v release vedle ostatních binárek*

Zde můžeme vidět drastický nárůst časové náročnosti s jen pár vrcholy, to je indikace exponenciálního růstu programu, proto nemůže být toto řešení aplikováno na rozsáhlejší problémy.

## Efektivnější řešení
NP-Hard problémy sice nelze řešit optimálně v polynomiálním čase, můžeme ale vytvořit takzvaný aproximační algoritmus, který se k optimálnímu řešení dostatečně přiblíží a zároveň běží v polynomiálním čase. 

K tomu, abychom tohoto dosáhli, potřebujeme nějakou greedy heuristiku, která umožní přeskočit co nejvíce operací, a zároveň vrátí dostatečně přesný výsledek.

Pro verze 2 a 3 jsem proto použil **Nearest neighbor heuristiku**, která běží v nejhorším případě v čase `O(n^2)` a vrátí cestu která je garantovaně maximálně 2x delší než optimální.

Dnes existují efektivnější algoritmy, s nižší časovou náročností i přesnější aproximací, pro účely tohoto úkolu však toto stačí. 

Jednovláknová verze tohoto algoritmu si nyní poradí s výrazně rozsáhlejšími grafy. 

| počet vrcholů | čas  |
| ------------- | ---- |
| 3             | 0ms  |
| 4             | 0ms  |
| 5             | 0ms  |
| 6             | 0ms  |
| 7             | 0ms  |
| 8             | 0ms  |
| 9             | 0ms  |
| 10            | 0ms  |
| 11            | 0ms  |
| 12            | 0ms  |
| 13            | 0ms  |
| 14            | 0ms  |
| 15            | 0ms  |
| 20            | 0ms  |
| 30            | 0ms  |
| 50            | 0ms  |
| 100           | 0ms  |
| 200           | 0ms  |
| 500           | 0ms  |
| 1000          | 1ms  |
| 10000         | 97ms |


Verze 3 je poté vícevláknová implementace tohoto algoritmu. Graf se nejdřív rozdělí na části, ty se pošlou do vláken, vyřeší a výsledky složí. Zrychlení tohoto algoritmu je vidět až na obrovských grafech, jako ten s 10000 vrcholy.

| počet vrcholů | čas |
| ------------- | --- |
| 10000         | 2ms    |

# Spuštění
Na github repositáři můžete najít release, kde jsou ke stažení binárky jednotlivých implementací, stejně tak jako programu pro generování grafů. 

po prohlednuti pozadovanych argumentu kteréhokoliv programu můžete spustit
```cmd
./verze1.exe --help
```
To ukáže command line argumenty, které program potřebuje k běhu.

Ty jsou u řešících algoritmů pouze cesta k souboru s grafy. Tento soubor se dá vygenerovat pomocí `graph_maker.exe`, tímto stylem:

```cmd
./graph_maker.exe 10 24 in.graph
```
To vygeneruje soubor `in.graph`, ve kterém bude 10 různých grafů, každý o 24ech vrcholech. 

Když chcete poté najít k těmto grafům cesty, spustíte
```cmd
./verze3.exe in.graph
```
