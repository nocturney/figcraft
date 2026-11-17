use super::{Context, FigApp, NodeId, RichText, Tokens, json, tr, trn, window};

/// The Rename layers dialog's fields.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BatchRenameUi {
    /// The layers, in stacking order (bottom first), as numbered.
    pub ids: Vec<NodeId>,
    /// Match: empty renames the whole name.
    pub find: String,
    pub regex: bool,
    /// Rename to.
    pub to: String,
    /// Where up numbers start and down numbers stop.
    pub start: u32,
}

/// Layers whose new names the dialog previews.
const PREVIEW_ROWS: usize = 5;

// Native controls keep a full-width row while choosing the visual reading edge.
// This mirrors the fields, preview and actions without reversing rename semantics.
fn rename_row<R>(ui: &mut egui::Ui, rtl: bool, f: impl FnOnce(&mut egui::Ui) -> R) -> R {
    ui.horizontal(|ui| if rtl { ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), f).inner } else { f(ui) }).inner
}

/// Open the Rename layers dialog for `ids` (several layers: Cmd/Ctrl+R, F2, the context menu).
pub(crate) fn open_batch_rename(app: &mut FigApp, ids: &[NodeId]) {
    let Ok(doc) = app.session.doc() else { return };
    let ids = figcraft_engine::cmd::edit::z_sorted(doc, ids);
    app.ui.batch_rename = Some(BatchRenameUi { ids, start: 1, ..Default::default() });
}

/// The rename the fields describe: a match replaced, or (no match) the whole name.
pub(crate) fn batch_rename_params(b: &BatchRenameUi) -> serde_json::Value {
    let mut p = json!({"ids": b.ids, "regex": b.regex, "start": b.start});
    if b.find.is_empty() {
        p["name"] = json!(b.to);
    } else {
        p["find"] = json!(b.find);
        p["replace"] = json!(b.to);
    }
    p
}

/// Rename layers: Match, Rename to with buttons for the current name and numbers, where numbers
/// start, a preview of the first new names, then one `object.batchRename`.
pub(super) fn batch_rename_dialog(app: &mut FigApp, ctx: &Context) {
    let Some(mut b) = app.ui.batch_rename.clone() else { return };
    let t = Tokens::get(ctx);
    let rtl = crate::i18n::right_to_left();
    let find = (!b.find.is_empty()).then_some(b.find.as_str());
    let rename = figcraft_engine::rename::BatchRename::new(find, b.regex, &b.to, find.is_none().then_some(b.to.as_str()), Some(f64::from(b.start)));
    let n = b.ids.len();
    let ready = rename.is_ok() && !(b.find.is_empty() && b.to.is_empty());
    let preview: Vec<(String, String)> = match (&rename, app.session.doc()) {
        (Ok(r), Ok(doc)) if ready => b
            .ids
            .iter()
            .enumerate()
            .take(PREVIEW_ROWS)
            .filter_map(|(k, i)| doc.node(*i).map(|node| (node.name.clone(), r.apply(&node.name, k, n))))
            .collect(),
        _ => Vec::new(),
    };
    let mut open = true;
    let mut cancel = false;
    let mut apply = false;
    window(tr("Rename layers"), ctx).id(egui::Id::unique("Rename layers")).open(&mut open).show(ctx, |ui| {
        ui.set_width(320.0);
        rename_row(ui, rtl, |ui| {
            ui.label(RichText::new(trn("{n} layers", n, &[])).color(t.text_secondary));
        });
        ui.add_space(6.0);
        let r = rename_row(ui, rtl, |ui| {
            ui.add(egui::TextEdit::singleline(&mut b.find).hint_text(tr("Match (empty: the whole name)")).desired_width(300.0))
        });
        crate::widgets::select_all_on_focus(ui, &r, &b.find);
        if rename_row(ui, rtl, |ui| crate::widgets::checkbox(ui, b.regex, tr("Regular expression"))) {
            b.regex = !b.regex;
        }
        let to = rename_row(ui, rtl, |ui| ui.add(egui::TextEdit::singleline(&mut b.to).hint_text(tr("Rename to")).desired_width(300.0)));
        crate::widgets::select_all_on_focus(ui, &to, &b.to);
        rename_row(ui, rtl, |ui| {
            for (label, code) in [(tr("Current name"), "$&"), (tr("Number ↑"), "$n"), (tr("Number ↓"), "$N")] {
                if ui.button(label).clicked() {
                    b.to.push_str(code);
                    to.request_focus();
                }
            }
        });
        rename_row(ui, rtl, |ui| {
            ui.label(tr("Start ascending from / stop descending at"));
            ui.add(egui::DragValue::new(&mut b.start).range(0..=1_000_000));
        });
        ui.add_space(8.0);
        match &rename {
            Err(e) => {
                rename_row(ui, rtl, |ui| {
                    ui.label(RichText::new(e).color(t.danger));
                });
            }
            Ok(_) => {
                for (old, new) in &preview {
                    rename_row(ui, rtl, |ui| {
                        // A right-to-left preview reads original, arrow, replacement from the right.
                        ui.label(RichText::new(old).color(t.text_secondary));
                        ui.label(if rtl { "←" } else { "→" });
                        ui.label(new);
                    });
                }
                if ready && n > PREVIEW_ROWS {
                    rename_row(ui, rtl, |ui| {
                        ui.label(RichText::new(trn("and {n} more", n - PREVIEW_ROWS, &[])).color(t.text_secondary));
                    });
                }
            }
        }
        ui.add_space(8.0);
        rename_row(ui, rtl, |ui| {
            cancel = crate::widgets::button(ui, tr("Cancel"), false, true).clicked();
            let enter = to.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            apply = (crate::widgets::button(ui, tr("Rename"), true, ready).clicked() || enter) && ready;
        });
    });
    if apply {
        app.run("object.batchRename", batch_rename_params(&b));
    }
    app.ui.batch_rename = (open && !cancel && !apply).then_some(b);
}
