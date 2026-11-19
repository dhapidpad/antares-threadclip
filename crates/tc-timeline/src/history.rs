use crate::commands::TimelineCommand;
use crate::model::ProjectModel;
use tc_core::Result;

pub struct TimelineHistory {
    undo_stack: Vec<Box<dyn TimelineCommand>>,
    redo_stack: Vec<Box<dyn TimelineCommand>>,
}

impl TimelineHistory {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn execute(
        &mut self,
        cmd: Box<dyn TimelineCommand>,
        project: &mut ProjectModel,
    ) -> Result<()> {
        cmd.execute(project)?;
        self.undo_stack.push(cmd);
        self.redo_stack.clear();
        Ok(())
    }

    pub fn undo(&mut self, project: &mut ProjectModel) -> Result<bool> {
        if let Some(cmd) = self.undo_stack.pop() {
            cmd.undo(project)?;
            self.redo_stack.push(cmd);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn redo(&mut self, project: &mut ProjectModel) -> Result<bool> {
        if let Some(cmd) = self.redo_stack.pop() {
            cmd.execute(project)?;
            self.undo_stack.push(cmd);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
