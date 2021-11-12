rez::rez! {
    vnější bedna rez;

    používej standardní_knihovna::kolekce::Slovník jako Slovník;

    vlastnost KlíčHodnota {
        funkce vlož(&já, klíč: Řetězec, hodnota: Řetězec);
        funkce čti(&já, klíč: Řetězec) -> Výsledek<Možná<&Řetězec>, Řetězec>;
    }

    statický měnitelný SLOVNÍK: Možná<Slovník<Řetězec, Řetězec>> = Nic;

    // Slib mi, že toto nikdy nepoužiješ mimo hlavní vlákno!
    struktura GlobálníSlovník;

    implementuj KlíčHodnota pro GlobálníSlovník {
        funkce vlož(&já, klíč: Řetězec, hodnota: Řetězec) {
            ať slovník = nebezpečné {
                SLOVNÍK.dostaň_nebo_vlož_s(Výchozí::výchozí)
            };

            slovník.vlož(klíč, hodnota);
        }

        funkce čti(&já, klíč: Řetězec) -> Výsledek<Možná<&Řetězec>, Řetězec> {
            jestli je Nějaký(slovník) = nebezpečné { SLOVNÍK.jako_odkaz() } {
                VPořádku(slovník.dostaň(&klíč))
            } jinak {
                Špatné("Ověření slovníku".konvertuj())
            }
        }
    }

    veřejná(v bedně) funkce možná(i: u32) -> Možná<Výsledek<u32, Řetězec>> {
        jestli i % 2 == 1 {
            jestli i == 42 {
                Nějaký(Špatný(Řetězec::z("Jejda!")))
            } jinak {
                Nějaké(VPořádku(33))
            }
        } jinak {
            Nic
        }
    }

    asynchronní funkce ukázka() { }

    asynchronní funkce ukázka2() {
        ukázka().vyčkej;
    }

    funkce hlavní() {
        ať měnitelné x = 31;

        odpovídá x {
            42 => {
                napiš_řádek!("Význam vesmíru je {}!", x)
            }
            _ => napiš_řádek!("Tak nic.")
        }

        pro i v 0..10 {
            ať hodnota = smyčka {
                vyskoč i;
            };

            zatímco x < hodnota {
                x += 1;
            }

            x = jestli je Nějaký(výsledek) = možná(i) {
                výsledek.rozbal()
            } jinak {
                12
            };
        }
    }

    #[povol(nedosažitelný_kód)]
    funkce když_se_nepodaří() {
        panika!("Panika!");
        ups!("Něco se pokazilo");
    }
}
