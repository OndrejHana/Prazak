# Prazak

Ukol Beta 4 pro predmet PV na SPSE Jecna

## Problem

jedna se o problem obchodniho cestujiciho (dale TSP)
To je NP-Hard problem, takze nemuze byt resen v polynomialnim case.

To lze videt casti verze1, ve ktere naleznete implementaci brute force algoritmu.

Ve verzi 2 a 3 naleznete reseni pomoci aproximacniho algoritmu- nearest neighbor.

Ta vyuziva greedy heuristiku a rozdeli problem na male podgrafy a pro ty naleze pouzitelne reseni.
Tento algoritmus ma runtime O(n^2)

Ve verzi 3 je problem rozdelen na podgrafy a resen na vice vlaknech soucasne.

Diky tomuto rozdeleni problemu muze byt problem distribuovan mezi vice vypocetnich jednotek, jak je videt ve verzi 4.
