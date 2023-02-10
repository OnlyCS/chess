use intuitive::{
    components::{experimental::modal::use_modal, *},
    *,
};

#[component(Promote)]
pub fn render(shown: bool) {
    let modal = use_modal();

    if *shown && !modal.is_shown() {
        modal.show(render! {
            Section(title: "Promote") {
                Text(text: "You are now eligible to promote!\n Choose one of the following: \n(B)ishop\n(K)night\n(Q)ueen\n(R)ook")
            }
        })
    } else if !*shown && modal.is_shown() {
        modal.hide();
    }

    render! {
        Section(title: "Promote") {
            Text(text: "No promotion available")
        }
    }
}
