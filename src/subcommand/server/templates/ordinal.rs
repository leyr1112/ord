use super::*;

#[derive(Display)]
pub(crate) struct OrdinalHtml {
  pub(crate) ordinal: Ordinal,
  pub(crate) blocktime: Blocktime,
}

impl Content for OrdinalHtml {
  fn title(&self) -> String {
    self.ordinal.degree().to_string()
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq, unindent::Unindent};

  #[test]
  fn ordinal_html() {
    assert_eq!(
      OrdinalHtml {
        ordinal: Ordinal(0),
        blocktime: Blocktime::Confirmed(0),
      }
      .to_string(),
      "
        <h1>Ordinal 0</h1>
        <dl>
          <dt>decimal</dt><dd>0.0</dd>
          <dt>degree</dt><dd>0°0′0″0‴</dd>
          <dt>name</dt><dd>nvtdijuwxlp</dd>
          <dt>block</dt><dd>0</dd>
          <dt>cycle</dt><dd>0</dd>
          <dt>epoch</dt><dd>0</dd>
          <dt>period</dt><dd>0</dd>
          <dt>offset</dt><dd>0</dd>
          <dt>rarity</dt><dd><span class=mythic>mythic</span></dd>
          <dt>time</dt><dd>1970-01-01 00:00:00</dd>
          <dt>percentile</dt><dd>0%</dd>
        </dl>
        <a>prev</a>
        <a href=/ordinal/1>next</a>
      "
      .unindent()
    );
  }

  #[test]
  fn ordinal_previous() {
    assert_eq!(
      OrdinalHtml {
        ordinal: Ordinal::LAST,
        blocktime: Blocktime::Confirmed(0),
      }
      .to_string(),
      "
        <h1>Ordinal 2099999997689999</h1>
        <dl>
          <dt>decimal</dt><dd>6929999.0</dd>
          <dt>degree</dt><dd>5°209999′1007″0‴</dd>
          <dt>name</dt><dd>a</dd>
          <dt>block</dt><dd>6929999</dd>
          <dt>cycle</dt><dd>5</dd>
          <dt>epoch</dt><dd>32</dd>
          <dt>period</dt><dd>3437</dd>
          <dt>offset</dt><dd>0</dd>
          <dt>rarity</dt><dd><span class=uncommon>uncommon</span></dd>
          <dt>time</dt><dd>1970-01-01 00:00:00</dd>
          <dt>percentile</dt><dd>100%</dd>
        </dl>
        <a href=/ordinal/2099999997689998>prev</a>
        <a>next</a>
      "
      .unindent()
    );
  }

  #[test]
  fn ordinal_next_and_previous() {
    assert_eq!(
      OrdinalHtml {
        ordinal: Ordinal(1),
        blocktime: Blocktime::Confirmed(0),
      }
      .to_string(),
      "
        <h1>Ordinal 1</h1>
        <dl>
          <dt>decimal</dt><dd>0.1</dd>
          <dt>degree</dt><dd>0°0′0″1‴</dd>
          <dt>name</dt><dd>nvtdijuwxlo</dd>
          <dt>block</dt><dd>0</dd>
          <dt>cycle</dt><dd>0</dd>
          <dt>epoch</dt><dd>0</dd>
          <dt>period</dt><dd>0</dd>
          <dt>offset</dt><dd>1</dd>
          <dt>rarity</dt><dd><span class=common>common</span></dd>
          <dt>time</dt><dd>1970-01-01 00:00:00</dd>
          <dt>percentile</dt><dd>0.000000000000047619047671428595%</dd>
        </dl>
        <a href=/ordinal/0>prev</a>
        <a href=/ordinal/2>next</a>
      "
      .unindent()
    );
  }
}