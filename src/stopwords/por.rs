use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Português (Portuguese)
pub static STOPWORDS_POR: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "a",
        "à",
        "acerca",
        "adeus",
        "agora",
        "aí",
        "ainda",
        "além",
        "algmas",
        "algo",
        "algumas",
        "alguns",
        "ali",
        "ambos",
        "ano",
        "anos",
        "antes",
        "ao",
        "aos",
        "apenas",
        "apoio",
        "apontar",
        "após",
        "aquela",
        "aquelas",
        "aquele",
        "aqueles",
        "aqui",
        "aquilo",
        "área",
        "as",
        "às",
        "assim",
        "até",
        "atrás",
        "através",
        "baixo",
        "bastante",
        "bem",
        "boa",
        "boas",
        "bom",
        "bons",
        "breve",
        "cá",
        "cada",
        "caminho",
        "catorze",
        "cedo",
        "cento",
        "certamente",
        "certeza",
        "cima",
        "cinco",
        "coisa",
        "com",
        "como",
        "comprido",
        "conhecido",
        "conselho",
        "contra",
        "corrente",
        "custa",
        "da",
        "dá",
        "dão",
        "daquela",
        "daquelas",
        "daquele",
        "daqueles",
        "dar",
        "das",
        "de",
        "debaixo",
        "demais",
        "dentro",
        "depois",
        "desde",
        "desligado",
        "dessa",
        "dessas",
        "desse",
        "desses",
        "desta",
        "destas",
        "deste",
        "destes",
        "deve",
        "devem",
        "deverá",
        "dez",
        "dezanove",
        "dezasseis",
        "dezassete",
        "dezoito",
        "dia",
        "diante",
        "direita",
        "diz",
        "dizem",
        "dizer",
        "do",
        "dois",
        "dos",
        "doze",
        "duas",
        "dúvida",
        "e",
        "é",
        "ela",
        "elas",
        "ele",
        "eles",
        "em",
        "embora",
        "enquanto",
        "então",
        "entre",
        "era",
        "és",
        "essa",
        "essas",
        "esse",
        "esses",
        "esta",
        "está",
        "estado",
        "estão",
        "estar",
        "estará",
        "estas",
        "estás",
        "estava",
        "este",
        "estes",
        "esteve",
        "estive",
        "estivemos",
        "estiveram",
        "estiveste",
        "estivestes",
        "estou",
        "eu",
        "exemplo",
        "faço",
        "falta",
        "fará",
        "favor",
        "faz",
        "fazeis",
        "fazem",
        "fazemos",
        "fazer",
        "fazes",
        "fazia",
        "fez",
        "fim",
        "final",
        "foi",
        "fomos",
        "for",
        "fora",
        "foram",
        "forma",
        "foste",
        "fostes",
        "fui",
        "geral",
        "grande",
        "grandes",
        "grupo",
        "há",
        "hoje",
        "hora",
        "horas",
        "iniciar",
        "inicio",
        "ir",
        "irá",
        "isso",
        "ista",
        "iste",
        "isto",
        "já",
        "lá",
        "lado",
        "ligado",
        "local",
        "logo",
        "longe",
        "lugar",
        "maior",
        "maioria",
        "maiorias",
        "mais",
        "mal",
        "mas",
        "máximo",
        "me",
        "meio",
        "menor",
        "menos",
        "mês",
        "meses",
        "mesmo",
        "meu",
        "meus",
        "mil",
        "minha",
        "minhas",
        "momento",
        "muito",
        "muitos",
        "na",
        "nada",
        "não",
        "naquela",
        "naquelas",
        "naquele",
        "naqueles",
        "nas",
        "nem",
        "nenhuma",
        "nessa",
        "nessas",
        "nesse",
        "nesses",
        "nesta",
        "nestas",
        "neste",
        "nestes",
        "nível",
        "no",
        "noite",
        "nome",
        "nos",
        "nós",
        "nossa",
        "nossas",
        "nosso",
        "nossos",
        "nova",
        "novas",
        "nove",
        "novo",
        "novos",
        "num",
        "numa",
        "número",
        "nunca",
        "o",
        "obra",
        "obrigada",
        "obrigado",
        "oitava",
        "oitavo",
        "oito",
        "onde",
        "ontem",
        "onze",
        "os",
        "ou",
        "outra",
        "outras",
        "outro",
        "outros",
        "para",
        "parece",
        "parte",
        "partir",
        "paucas",
        "pegar",
        "pela",
        "pelas",
        "pelo",
        "pelos",
        "perto",
        "pessoas",
        "pode",
        "pôde",
        "podem",
        "poder",
        "poderá",
        "podia",
        "põe",
        "põem",
        "ponto",
        "pontos",
        "por",
        "porque",
        "porquê",
        "posição",
        "possível",
        "possivelmente",
        "posso",
        "pouca",
        "pouco",
        "poucos",
        "povo",
        "primeira",
        "primeiras",
        "primeiro",
        "primeiros",
        "promeiro",
        "própria",
        "próprias",
        "próprio",
        "próprios",
        "próxima",
        "próximas",
        "próximo",
        "próximos",
        "puderam",
        "quáis",
        "qual",
        "qualquer",
        "quando",
        "quanto",
        "quarta",
        "quarto",
        "quatro",
        "que",
        "quê",
        "quem",
        "quer",
        "quereis",
        "querem",
        "queremas",
        "queres",
        "quero",
        "questão",
        "quieto",
        "quinta",
        "quinto",
        "quinze",
        "relação",
        "sabe",
        "sabem",
        "saber",
        "são",
        "se",
        "segunda",
        "segundo",
        "sei",
        "seis",
        "sem",
        "sempre",
        "ser",
        "seria",
        "sete",
        "sétima",
        "sétimo",
        "seu",
        "seus",
        "sexta",
        "sexto",
        "sim",
        "sistema",
        "sob",
        "sobre",
        "sois",
        "somente",
        "somos",
        "sou",
        "sua",
        "suas",
        "tal",
        "talvez",
        "também",
        "tanta",
        "tantas",
        "tanto",
        "tão",
        "tarde",
        "te",
        "tem",
        "têm",
        "temos",
        "tempo",
        "tendes",
        "tenho",
        "tens",
        "tentar",
        "tentaram",
        "tente",
        "tentei",
        "ter",
        "terceira",
        "terceiro",
        "teu",
        "teus",
        "teve",
        "tipo",
        "tive",
        "tivemos",
        "tiveram",
        "tiveste",
        "tivestes",
        "toda",
        "todas",
        "todo",
        "todos",
        "trabalhar",
        "trabalho",
        "três",
        "treze",
        "tu",
        "tua",
        "tuas",
        "tudo",
        "último",
        "um",
        "uma",
        "umas",
        "uns",
        "usa",
        "usar",
        "vai",
        "vais",
        "valor",
        "vão",
        "vários",
        "veja",
        "vem",
        "vêm",
        "vens",
        "ver",
        "verdade",
        "verdadeiro",
        "vez",
        "vezes",
        "viagem",
        "vindo",
        "vinte",
        "você",
        "vocês",
        "vos",
        "vós",
        "vossa",
        "vossas",
        "vosso",
        "vossos",
        "zero",
    ]
    .iter()
    .cloned()
    .collect()
});
