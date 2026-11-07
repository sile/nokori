bkl
===

- backlog management cli tool
- markdown file based
- commands:
  - bkl init
    - (optional) add exclude to cargo.toml if exists
    - create backlog/ dir and backlog/README.md
    - README.md niha saiteigen no naiyou nomiwo kijutusuru 
      - bkl version kousin no tokino maintenance cost wo sagerutame
    - kawarini nkl new de tukuru template file no naiyou wo self descriptive nis teioku
      - kaku section ni <!-- TODO: ... --> wo kaiteoite, sono section de yarubekikoto / kiwotukerubekikoto wo kaiteoku
      - riyousyaha, section no kisaiga owattara, sono TODO wo kesu (koremo TODO: ni tejun tosite kaiteoku)
  - bkl new (or create) TYPE
    - ITEM_TYPE=rfc,bug,plan,memo
    - rfc: sin kinou ya kyodou henkou no teian
      - jissou syousai ha hukumesuginai (soreha jissoujini kimereba iinode, jissouni tuiteha hontou ni hituyouna jouhounomiwo kisaisusuru)
      - rule
        - dekirudake self-contained na naiyounisuru (hokano repository tokakarano youkyuudemo, dekirudake ippanntekina youkyuu ni otosikonnde kisaisuru)
    - bug: bug syusei
    - plan: item no taiouga omoi baaiya, hukusuu no phase ni wakete torikumu baai(kottiha umbrella teki) niha, kono item wo tukuttemo ii
      - aruteido gutaitekina tejun ya nagarewo kisaisurutameno mono
      - tugino major release madeno milestone, tekina tukaiktademoii
    - memo: free format no memo
      - jissaini sagyouni hairumaeniha, hokano item ni otosikon de kara torikumu noga suisyou sareru
      - goku keibina monoha reigai
    - TODO: hokano kubun ga hituyouka, ya ueno bunrui nado ga tekisetuka, ha kentousuru
  - bkl finish or close or settle or resolve else
    - file no idou to status henkou wo yarutameno command?
      - status ni awaseru nara 'accept' tokamo ari?
    - mada syousai ha humei (somosomo hituyouka douka, tokamo)
- status:
  - TODO
  - ITEM_TYPE goto ni kotonaru status wo motu?
  - jissouni tyakusyu igo ha, kihontekini kaku ITEM no kizon no nakami ha syuuseisinai (tuiki ga omo)
- dir structure:
  - backlog/
    - README.md
    - done/ (if need)
    - pending/ (if need)


