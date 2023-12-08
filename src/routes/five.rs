use actix_web::{get, HttpResponse, Responder};

const GRINCH_SONG: &str = r#"You're a mean one, Mr. Grinch
You really are a heel,
You're as cuddly as a cactus, you're as charming as an eel, Mr. Grinch,
You're a bad banana with a greasy black peel!

You're a monster, Mr. Grinch,
Your heart's an empty hole,
Your brain is full of spiders, you've got garlic in your soul, Mr. Grinch,
I wouldn't touch you with a thirty-nine-and-a-half foot pole!

You're a vile one, Mr. Grinch,
You have termites in your smile,
You have all the tender sweetness of a seasick crocodile, Mr. Grinch,
Given the choice between the two of you I'd take the seasick crocodile!

You're a foul one, Mr. Grinch,
You're a nasty wasty skunk,
Your heart is full of unwashed socks, your soul is full of gunk, Mr. Grinch,
The three words that best describe you are as follows, and I quote,
Stink! Stank! Stunk!

You're a rotter, Mr. Grinch,
You're the king of sinful sots,
Your heart's a dead tomato splotched with moldy purple spots, Mr. Grinch,
Your soul is an appalling dump heap overflowing with the most disgraceful
assortment of deplorable rubbish imaginable mangled up in tangled up knots!

You nauseate me, Mr. Grinch,
With a nauseous super-naus
You're a crooked jerky jockey and you drive a crooked hoss, Mr. Grinch,
You're a three decker sauerkraut and toadstool sandwich with arsenic sauce!"#;

#[get("/5")]
pub async fn grinch() -> impl Responder {
    HttpResponse::Ok().body(GRINCH_SONG)
}
