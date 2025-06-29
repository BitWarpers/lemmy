use actix_web::web::{Data, Json};
use lemmy_api_utils::context::LemmyContext;
use lemmy_db_schema::{
  source::comment_reply::{CommentReply, CommentReplyUpdateForm},
  traits::Crud,
};
use lemmy_db_views_inbox_combined::api::MarkCommentReplyAsRead;
use lemmy_db_views_local_user::LocalUserView;
use lemmy_db_views_site::api::SuccessResponse;
use lemmy_utils::error::{LemmyErrorType, LemmyResult};

pub async fn mark_reply_as_read(
  data: Json<MarkCommentReplyAsRead>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<SuccessResponse>> {
  let comment_reply_id = data.comment_reply_id;
  let read_comment_reply = CommentReply::read(&mut context.pool(), comment_reply_id).await?;

  if local_user_view.person.id != read_comment_reply.recipient_id {
    Err(LemmyErrorType::CouldntUpdateComment)?
  }

  let comment_reply_id = read_comment_reply.id;
  let read = Some(data.read);

  CommentReply::update(
    &mut context.pool(),
    comment_reply_id,
    &CommentReplyUpdateForm { read },
  )
  .await?;

  Ok(Json(SuccessResponse::default()))
}
