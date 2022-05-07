use crate::communication::BuildMetadata;
use crate::document::PortfolioMessageHandler;
use crate::layout::{layout_message::LayoutTarget, widgets::PropertyHolder};
use crate::message_prelude::*;

use super::*;

#[derive(Debug, Default, Clone)]
pub struct DialogMessageHandler {
	new_document_dialog: NewDocument,
}

impl MessageHandler<DialogMessage, (&BuildMetadata, &PortfolioMessageHandler)> for DialogMessageHandler {
	#[remain::check]
	fn process_action(&mut self, message: DialogMessage, (build_metadata, portfolio): (&BuildMetadata, &PortfolioMessageHandler), responses: &mut VecDeque<Message>) {
		#[remain::sorted]
		match message {
			#[remain::unsorted]
			DialogMessage::NewDocumentDialog(message) => self.new_document_dialog.process_action(message, (), responses),

			DialogMessage::CloseAllDocumentsWithConfirmation => {
				let dialog = dialogs::CloseAllDocuments;
				dialog.register_properties(responses, LayoutTarget::DialogDetails);
				responses.push_back(FrontendMessage::DisplayDialog { icon: "Copy".to_string() }.into());
			}
			DialogMessage::CloseDialogAndThen { followup } => {
				responses.push_back(FrontendMessage::DisplayDialogDismiss.into());
				responses.push_back(*followup);
			}
			DialogMessage::DisplayDialogError { title, description } => {
				let dialog = dialogs::Error { title, description };
				dialog.register_properties(responses, LayoutTarget::DialogDetails);
				responses.push_back(FrontendMessage::DisplayDialog { icon: "Warning".to_string() }.into());
			}
			DialogMessage::RequestAboutGraphiteDialog => {
				let about_graphite = AboutGraphite {
					build_metadata: build_metadata.clone(),
				};
				about_graphite.register_properties(responses, LayoutTarget::DialogDetails);
				responses.push_back(FrontendMessage::DisplayDialog { icon: "GraphiteLogo".to_string() }.into());
			}
			DialogMessage::RequestComingSoonDialog { issue } => {
				let coming_soon = ComingSoon { issue };
				coming_soon.register_properties(responses, LayoutTarget::DialogDetails);
				responses.push_back(FrontendMessage::DisplayDialog { icon: "Warning".to_string() }.into());
			}
			DialogMessage::RequestNewDocumentDialog => {
				self.new_document_dialog = NewDocument {
					name: portfolio.generate_new_document_name(),
					infinite: true,
					dimensions: glam::UVec2::new(1920, 1080),
				};
				self.new_document_dialog.register_properties(responses, LayoutTarget::DialogDetails);
				responses.push_back(FrontendMessage::DisplayDialog { icon: "File".to_string() }.into());
			}
		}
	}

	advertise_actions!(DialogMessageDiscriminant;RequestNewDocumentDialog,CloseAllDocumentsWithConfirmation);
}