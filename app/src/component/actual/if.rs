use serde::Deserialize;

use crate::{
    component::{Component, ComponentAction, RenderContext},
    system::Data,
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Expression {
    Variable {
        name: String,
    },
    Const {
        #[serde(rename = "$value")]
        value: String,
    },
}

impl Expression {
    fn evaluate(&self, context: &RenderContext) -> eyre::Result<Data> {
        match self {
            Expression::Variable { name } => context
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| eyre::eyre!("THere are no variable named {}", name)),
            Expression::Const { value } => Ok(Data::String(value.clone())),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct If {
    cond: Expression,
    then: Box<Component>,
    #[serde(rename = "else")]
    otherwise: Option<Box<Component>>,
}

impl ComponentAction for If {
    fn setup<'a>(
        &'a mut self,
    ) -> eyre::Result<
        Box<dyn FnOnce(&mut crate::component::SetupContext) -> eyre::Result<()> + Send + 'a>,
    > {
        let then_fn = self.then.setup()?;
        let else_fn = self
            .otherwise
            .as_mut()
            .map(|otherwise| otherwise.setup())
            .transpose()?;

        Ok(Box::new(|context| {
            then_fn(context)?;
            if let Some(else_fn) = else_fn {
                else_fn(context)?;
            }

            Ok(())
        }))
    }

    fn update(&mut self, context: &mut crate::component::UpdateContext) -> eyre::Result<()> {
        self.then.update(context)?;
        if let Some(otherwise) = self.otherwise.as_mut() {
            otherwise.update(context)?;
        }

        Ok(())
    }

    fn render(&mut self, context: &RenderContext) -> eyre::Result<usvg::Node> {
        let cond = self.cond.evaluate(context)?.as_bool()?;

        match (cond, self.then.as_mut(), self.otherwise.as_mut()) {
            (true, then, _) => then.render(context),
            (false, _, Some(otherwise)) => otherwise.render(context),
            _ => Ok(usvg::Node::new(usvg::NodeKind::Group(
                usvg::Group::default(),
            ))),
        }
    }
}
