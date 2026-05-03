use std::fmt;

use bytes::{BufMut, BytesMut};

use crate::api::requests::{ReqDescribeTopicPartitions, RequestHeader};
use crate::api::responses::traits::{ResponseBody, ResponseBytes};
use crate::error::KafkaError;

#[derive(Debug)]
pub struct DescribeTopicBody {
    pub throttle_time: u32,
    pub topics: Vec<TopicV0Entry>,
    pub next_cursor: Option<u8>,
    pub tag_buffer: u8,
}

#[derive(Debug)]
pub struct TopicV0Entry {
    pub error_code: u16,
    pub name: String,
    pub topic_id: [u8; 16],
    pub is_internal: bool,
    pub partitions: Vec<PartitionV0Entry>,
    pub topic_authorized_operations: u32,
    pub tag_buffer: u8,
}

#[derive(Debug)]
pub struct PartitionV0Entry {
    pub error_code: u16,
    pub index: u32,
    pub leader_id: u32,
    pub leader_epoch: u32,
    pub replica_nodes: Vec<ResponseTopicNodes>,
    pub isr_nodes: Vec<ResponseTopicISRNode>,
    pub elligible_leader_replicas: u8,
    pub last_known_elr: u8,
    pub offline_replicas: u8,
    pub tag_buffer: u8,
}

#[derive(Debug)]
pub struct ResponseTopicNodes(pub u32);

#[derive(Debug)]
pub struct ResponseTopicISRNode(pub u32);

impl ResponseBytes for DescribeTopicBody {
    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::new();
        bytes.put_u32(self.throttle_time);
        bytes.put_u8(self.topics.len() as u8 + 1);

        let topics_bytes = self.topics.iter().fold(BytesMut::new(), |mut acc, topic| {
            acc.extend(topic.to_bytes().iter());
            acc
        });

        bytes.extend(topics_bytes.iter());
        bytes.put_u8(self.next_cursor.unwrap_or(0xff));
        bytes.put_u8(self.tag_buffer);

        bytes
    }
}

impl ResponseBody<ReqDescribeTopicPartitions> for DescribeTopicBody {
    fn from_request(_: &RequestHeader, req_body: &ReqDescribeTopicPartitions) -> Self {
        let mut topics = Vec::new();

        for topic in &req_body.topics.1 {
            topics.push(TopicV0Entry {
                error_code: KafkaError::UnknownTopicOrPartition(0).into_error_code(),
                name: topic.name.1.clone(),
                topic_id: [0u8; 16],
                is_internal: false,
                partitions: Vec::new(),
                topic_authorized_operations: 0x00000df8,
                tag_buffer: 0,
            });
        }

        Self {
            throttle_time: 0,
            topics,
            next_cursor: None,
            tag_buffer: req_body.tag_buffer.0,
        }
    }
}

impl fmt::Display for DescribeTopicBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let topics_len = self.topics.len();
        writeln!(f, "")?;
        writeln!(f, " - Body")?;
        writeln!(f, "  - ThrottleTimeMs ({})", self.throttle_time)?;
        writeln!(f, "  - Topics")?;
        writeln!(f, "   - Length ({topics_len})")?;
        let mut iter_topics = self.topics.iter().enumerate();
        while let Some((idx, topic)) = iter_topics.next() {
            writeln!(f, "   - Topics[{idx}]")?;
            topic.fmt(f)?;
        }
        writeln!(f, "  - Cursor ({:?})", self.next_cursor)?;
        writeln!(f, "  - TagBuffer ({})", self.tag_buffer)
    }
}

impl fmt::Display for TopicV0Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "    - ErrorCode ({})", self.error_code)?;
        writeln!(f, "    - Name ({})", self.name)?;
        writeln!(f, "    - UUID ({:?})", self.topic_id)?;
        writeln!(f, "    - IsInternal ({})", self.is_internal)?;

        writeln!(f, "    - Partitions")?;
        writeln!(f, "     - Length ({})", self.partitions.len())?;
        let mut iter_partition = self.partitions.iter().enumerate();
        while let Some((idx, partition)) = iter_partition.next() {
            writeln!(f, "     - Partition[{idx}]")?;
            partition.fmt(f)?;
        }
        writeln!(
            f,
            "    - TopicAuthorizedOperations ({})",
            self.topic_authorized_operations
        )?;
        writeln!(f, "    - TagBuffer ({})", self.tag_buffer)
    }
}

impl fmt::Display for PartitionV0Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "      - ErrorCode ({})", self.error_code)?;
        writeln!(f, "      - PartitionIdx ({})", self.index)?;
        writeln!(f, "      - LeaderId ({})", self.leader_id)?;
        writeln!(f, "      - LeaderEpoch ({})", self.leader_epoch)?;
        writeln!(f, "      - ReplicaNodes ({:?})", self.replica_nodes)?;
        writeln!(f, "      - ISRNodes ({:?})", self.isr_nodes)?;
        writeln!(
            f,
            "      - ElligibleLeaderReplicas ({:?})",
            self.elligible_leader_replicas
        )?;
        writeln!(f, "      - LastKnownELR ({:?})", self.last_known_elr)?;
        writeln!(f, "      - OfflineReplicas ({:?})", self.offline_replicas)?;
        writeln!(f, "      - TagBuffer ({:?})", self.isr_nodes)
    }
}

impl ResponseBytes for TopicV0Entry {
    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::new();
        bytes.put_u16(self.error_code);
        bytes.put_u8((self.name.len() as u8) + 1);
        bytes.extend(self.name.bytes());
        bytes.put(&self.topic_id[..]);
        bytes.put_u8(self.is_internal.into());

        bytes.put_u8((self.partitions.len() as u8) + 1);
        let partition_bytes = self
            .partitions
            .iter()
            .fold(BytesMut::new(), |mut acc, topic| {
                acc.extend(topic.to_bytes());
                acc
            });

        bytes.put(partition_bytes);
        bytes.put_u32(self.topic_authorized_operations);
        bytes.put_u8(self.tag_buffer);

        bytes
    }
}

impl ResponseBytes for PartitionV0Entry {
    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::new();
        bytes.put_u16(self.error_code);
        bytes.put_u32(self.index);
        bytes.put_u32(self.leader_id);
        bytes.put_u32(self.leader_epoch);

        bytes.put_u8(self.replica_nodes.len() as u8 + 1);
        let topic_nodes = self
            .replica_nodes
            .iter()
            .fold(BytesMut::new(), |mut acc, topic_node| {
                acc.extend(topic_node.to_bytes());
                acc
            });
        bytes.extend(topic_nodes);

        let isr_nodes = self
            .isr_nodes
            .iter()
            .fold(BytesMut::new(), |mut acc, isr_node| {
                acc.extend(isr_node.to_bytes());
                acc
            });
        bytes.extend(isr_nodes);
        bytes.put_u8(0);
        bytes.put_u8(0);
        bytes.put_u8(0);
        bytes.put_u8(self.tag_buffer);

        bytes
    }
}

impl ResponseBytes for ResponseTopicNodes {
    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::with_capacity(4);
        bytes.put_u32(self.0);
        bytes
    }
}

impl ResponseBytes for ResponseTopicISRNode {
    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::with_capacity(4);
        bytes.put_u32(self.0);
        bytes
    }
}

impl TopicV0Entry {
    pub fn size(&self) -> u32 {
        let partition_sum = self.partitions.iter().map(|part| part.size()).sum::<u32>();
        26 + self.name.len() as u32 + partition_sum
    }
}

impl PartitionV0Entry {
    pub fn size(&self) -> u32 {
        let replica_nodes_sum = self.replica_nodes.iter().map(|_| 4u32).sum::<u32>();
        let isr_nodes_sum = self.isr_nodes.iter().map(|_| 4u32).sum::<u32>();
        20 + replica_nodes_sum + isr_nodes_sum
    }
}
